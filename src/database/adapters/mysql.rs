use crate::database::database_config::MysqlConfig;
use crate::dylibs_binding::mysqlclient::{
    mysql, mysql_close, mysql_errno, mysql_error, mysql_init, mysql_ping, mysql_real_connect,
    MYSQL_DEFAULT_PORT,
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
