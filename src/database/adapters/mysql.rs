use crate::database::database_config::MysqlConfig;
use crate::database::models::user::{CrudUserDao, User, Username};
use crate::dylibs_binding::mysqlclient::{
    my_ulonglong, mysql, mysql_affected_rows, mysql_close, mysql_errno, mysql_error, mysql_init,
    mysql_ping, mysql_query, mysql_real_connect, MYSQL_DEFAULT_PORT,
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
            libc::exit(1);
        }
    }

    /// sql string without nul byte
    pub fn query(&self, sql: &str) {
        let sql = CString::new(sql).unwrap();
        let ret = unsafe { mysql_query(self.connection, sql.as_ptr().cast()) };
        if ret != 0 {
            self.print_last_mysql_error_and_exit();
        }
    }

    /// if delete the whole tables, the affected rows would be zero
    pub fn affected_rows(&self) -> my_ulonglong {
        unsafe { mysql_affected_rows(self.connection) }
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
    let config = crate::database::database_config::Config::default();
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
        self.query("drop table if exists users");
        self.query("create table if not exists users(user_id tinyint unsigned not null primary key, username varchar(7) not null)");
        for user_id in 0..Self::Model::LEN {
            let user_id = user_id as u8;
            let user = Self::Model::new(user_id);
            let username = String::from_utf8_unchecked(user.username.to_vec());
            // use prepare statement to insert is better
            let insert_sql = format!(
                "insert into users(user_id, username) values({}, '{}')",
                user.user_id, username
            );
            self.query(&insert_sql);
            assert_eq!(self.affected_rows(), 1);
        }
    }

    unsafe fn select_all(&self) -> Vec<Self::Model> {
        todo!()
    }

    unsafe fn find_user_by_id(&self, user_id: u8) -> Self::Model {
        todo!()
    }

    unsafe fn update_username_by_id(&self, user_id: u8, username: Username) {
        let username = String::from_utf8_unchecked(username.to_vec());
        // use prepare statement to update is better
        let insert_sql = format!(
            "update users set username='{}' where user_id={}",
            username, user_id
        );
        self.query(&insert_sql);
        if self.affected_rows() == 0 {
            panic!("user_id={} not found", user_id);
        }
    }
}

#[test]
fn test_insert_sample_data() {
    let config = crate::database::database_config::Config::default();
    let mysql_conn = MysqlConnection::new(config.mysql);
    unsafe {
        mysql_conn.insert_sample_data();
    }
}

#[test]
fn test_update_username_by_id() {
    let config = crate::database::database_config::Config::default();
    let mysql_conn = MysqlConnection::new(config.mysql);
    unsafe {
        mysql_conn.insert_sample_data();
        mysql_conn.update_username_by_id(3, *b"tuesday");
    }
}
