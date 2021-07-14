use crate::database::database_config::MysqlConfig;
use crate::database::models::user::{CrudUserDao, User, Username};
use crate::dylibs_binding::mysqlclient::{
    my_ulonglong, mysql, mysql_affected_rows, mysql_close, mysql_errno, mysql_error,
    mysql_fetch_row, mysql_field_count, mysql_free_result, mysql_init, mysql_num_rows, mysql_ping,
    mysql_query, mysql_real_connect, mysql_store_result, MYSQL_DEFAULT_PORT,
};
use std::ffi::CString;

pub struct MysqlConnection {
    connection: *mut mysql,
}

impl MysqlConnection {
    #[must_use]
    pub fn new(config: MysqlConfig) -> Self {
        let connection = unsafe {
            #[allow(clippy::zero_ptr)]
            mysql_init(0 as *mut mysql)
        };
        let mysql_conn = Self { connection };
        /*
        you can call mysql_options() between mysql_init and mysql_real_connect
        - MYSQL_INIT_COMMAND, like sqlx's after_connect() API
        - MYSQL_OPT_RECONNECT
        - MYSQL_OPT_CONNECT_TIMEOUT
        - MYSQL_OPT_COMPRESS(compress on transfer)
        ...
        */
        let username = CString::new(config.username).unwrap();
        let password = CString::new(config.password).unwrap();
        let db_name = CString::new(config.db_name).unwrap();
        let connect_result = unsafe {
            mysql_real_connect(
                mysql_conn.connection,
                "localhost\0".as_ptr().cast(),
                username.as_ptr().cast(),
                password.as_ptr().cast(),
                db_name.as_ptr().cast(),
                MYSQL_DEFAULT_PORT,
                std::ptr::null(),
                0,
            )
        };
        if connect_result.is_null() {
            mysql_conn.print_last_mysql_error_and_exit();
        }
        mysql_conn
    }

    /// return true if ping success
    #[must_use]
    pub fn ping(&self) -> bool {
        unsafe { mysql_ping(self.connection) == 0 }
    }

    pub fn print_last_mysql_error_and_exit(&self) {
        unsafe {
            libc::printf(
                "mysql errno=%d, err_msg=%s\n\0".as_ptr().cast(),
                mysql_errno(self.connection),
                mysql_error(self.connection),
            );
            libc::exit(libc::EXIT_FAILURE);
        }
    }

    /// sql string without nul byte
    #[allow(clippy::must_use_candidate)]
    pub fn query<T: std::str::FromStr>(&self, sql: &str) -> Option<Vec<T>> {
        let is_select_statement = sql.contains("select") || sql.contains("SELECT");
        let sql = CString::new(sql).unwrap();
        let ret = unsafe { mysql_query(self.connection, sql.as_ptr().cast()) };
        if ret != 0 {
            self.print_last_mysql_error_and_exit();
        }
        // only select statement has return value
        if !is_select_statement {
            return None;
        }

        let res_ptr = unsafe { mysql_store_result(self.connection) };
        if res_ptr.is_null() {
            self.print_last_mysql_error_and_exit();
        }

        let num_rows = unsafe { mysql_num_rows(res_ptr) } as usize;
        let mut records = vec![];
        // loop times is mysql_num_rows()
        loop {
            // type of sql_row is Vec<Vec<u8>>
            let sql_row = unsafe { mysql_fetch_row(res_ptr) };
            if sql_row.is_null() {
                break;
            }

            // csv format in string
            let mut row_bytes = Vec::<u8>::new();

            let fields = unsafe { mysql_field_count(self.connection) };
            for index in 0..fields {
                let field_str = unsafe {
                    /*
                    Reference: BLP aka *beginning linux programming 4th edition*
                    BLP page 284(pdf_317):
                    > (note for dbm_fetch)
                    >
                    > The actual data may still be held in local storage space inside the dbm library
                    >
                    > and must be copied into program variables before any further dbm functions are called

                    BLP page 431(pdf_374):
                    > mysql_error, which provides a meaningful text message instead.
                    >
                    > The message text is written to some internal static memory space,
                    >
                    > so you need tocopy it elsewhere if you want to save the error text
                    */
                    // copy strdup bytes from mysql dylib
                    // or `Vec::from_raw_parts` and mem::forget
                    // when from_raw_parts drop, would dealloc bytes in mysql dylib cause memory error, must manual forget to drop
                    // let bytes = *sql_row.add(index as usize);
                    let bytes = libc::strdup(*sql_row.add(index as usize)); // or strcpy, or std::ptr::copy
                    let bytes_len = libc::strlen(bytes);
                    String::from_raw_parts(bytes.cast(), bytes_len, bytes_len)
                };
                row_bytes.extend(field_str.into_bytes());
                row_bytes.push(b',');
            }

            if !row_bytes.is_empty() {
                // pop last comma
                row_bytes.pop().unwrap();
                // BLP 书上例子是用 sscanf 解析每一个字段的值
                let row_str = unsafe { String::from_utf8_unchecked(row_bytes) };
                #[allow(clippy::match_wild_err_arm)]
                let record = match row_str.parse::<T>() {
                    Ok(record) => record,
                    Err(_) => panic!("parse error"),
                };
                records.push(record);
            }
        }
        unsafe {
            mysql_free_result(res_ptr);
        }
        assert_eq!(records.len(), num_rows);
        Some(records)
    }

    /// if delete the whole tables, the affected rows would be zero
    #[must_use]
    pub fn affected_rows(&self) -> my_ulonglong {
        unsafe { mysql_affected_rows(self.connection) }
    }

    /// last_insert_id in current connection(current thread?)
    #[must_use]
    pub fn last_insert_id(&self) -> libc::c_ulong {
        self.query::<libc::c_ulong>("select last_insert_id()")
            .unwrap()[0]
    }
}

impl Drop for MysqlConnection {
    fn drop(&mut self) {
        unsafe {
            mysql_close(self.connection);
        }
    }
}

/**
mysql's general_log output:
```text
/usr/bin/mariadbd, Version: 10.5.10-MariaDB (Arch Linux). started with:
Tcp port: 3306  Unix socket: /run/mysqld/mysqld.sock
Time                Id Command  Argument
210711 21:19:32     39 Connect  w@localhost on test using Socket
                    39 Quit
```
*/
#[test]
fn test_mysql_connect_and_ping() {
    let config = crate::database::database_config::Config::load_production_config();
    let mysql_conn = MysqlConnection::new(config.mysql);
    assert!(mysql_conn.ping());
}

/// err_msg=MySQL server has gone away
#[test]
fn test_error_mysql_server_has_gone() {
    let connection = unsafe {
        #[allow(clippy::zero_ptr)]
        mysql_init(0 as *mut mysql)
    };
    let ret = unsafe { mysql_ping(connection) };
    assert_ne!(ret, 0);
    unsafe {
        libc::printf(
            "mysql errno=%d, err_msg=%s\n\0".as_ptr().cast(),
            mysql_errno(connection),
            mysql_error(connection),
        );
    }
}

impl CrudUserDao for MysqlConnection {
    type Model = User;

    unsafe fn insert_sample_data(&self) {
        self.query::<bool>("drop table if exists users");
        self.query::<bool>("create table if not exists users(user_id tinyint unsigned not null primary key, username varchar(7) not null)");
        for user_id in 0..Self::Model::LEN {
            let user_id = user_id as u8;
            let user = Self::Model::new(user_id);
            let username = String::from_utf8_unchecked(user.username.to_vec());
            // use prepare statement to insert is better
            let insert_sql = format!(
                "insert into users(user_id, username) values({}, '{}')",
                user.user_id, username
            );
            self.query::<bool>(&insert_sql);
            assert_eq!(self.affected_rows(), 1);
            // because our user_id is set manual, and not AUTO_INCREMENT
            // so last_insert_id() would be zero after insert(because last insert id not using AUTO_INCREMENT)
        }
    }

    unsafe fn select_all(&self) -> Vec<Self::Model> {
        self.query("select user_id, username from users").unwrap()
    }

    unsafe fn find_user_by_id(&self, user_id: u8) -> Self::Model {
        let sql = format!(
            "select user_id, username from users where user_id={}",
            user_id
        );
        self.query(&sql).unwrap()[0]
    }

    unsafe fn update_username_by_id(&self, user_id: u8, username: Username) {
        let username = String::from_utf8_unchecked(username.to_vec());
        // use prepare statement to update is better
        let insert_sql = format!(
            "update users set username='{}' where user_id={}",
            username, user_id
        );
        self.query::<bool>(&insert_sql);
        if self.affected_rows() == 0 {
            panic!("user_id={} not found", user_id);
        }
    }
}

#[test]
fn test_insert_sample_data() {
    let config = crate::database::database_config::Config::load_production_config();
    let mysql_conn = MysqlConnection::new(config.mysql);
    unsafe {
        mysql_conn.insert_sample_data();
    }
}

#[test]
fn test_update_username_by_id() {
    let config = crate::database::database_config::Config::load_production_config();
    let mysql_conn = MysqlConnection::new(config.mysql);
    unsafe {
        mysql_conn.insert_sample_data();
        mysql_conn.update_username_by_id(3, *b"tuesday");
    }
}

#[test]
fn test_query_with_generic() {
    let config = crate::database::database_config::Config::load_production_config();
    let mysql_conn = MysqlConnection::new(config.mysql);
    unsafe {
        mysql_conn.insert_sample_data();
    }
    let user_id = mysql_conn
        .query::<u8>(
            "\
            SELECT user_id \
            FROM users \
            WHERE username='user_01'",
        )
        .unwrap()[0];
    assert_eq!(user_id, 1);
    let user = mysql_conn
        .query::<User>(
            "\
            SELECT user_id, username \
            FROM users \
            WHERE username='user_01'",
        )
        .unwrap()[0];
    assert_eq!(user.user_id, 1);
}

#[test]
fn test_mysql_database() {
    let config = crate::database::database_config::Config::load_production_config();
    let db_adapter = MysqlConnection::new(config.mysql);
    crate::database::models::user::test_user_crud(&db_adapter);
}
