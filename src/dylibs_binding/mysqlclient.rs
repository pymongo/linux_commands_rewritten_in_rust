use libc::{c_char, c_int, c_uint};

/**
> gcc -lmysqlclient mysql_conn_example.c

```text
// mysql_conn_example.c
#include <stdio.h>
#include <mysql/mysql.h>

int main(int argc, char *argv[]) {
    MYSQL *conn_ptr = mysql_init(NULL);
    conn_ptr = mysql_real_connect(conn_ptr, "localhost",
        "w",
        "w",
        "test",
    0, NULL, 0);
    if (conn_ptr) {
        printf("Connection success\n");
    } else {
        printf("Connection failed\n");
    }
    mysql_close(conn_ptr);
    return 0;
}
```
*/
#[link(name = "mysqlclient")]
extern "C" {
    pub type mysql;
    pub fn mysql_errno(connection: *mut mysql) -> c_uint;
    pub fn mysql_error(connection: *mut mysql) -> *const c_char;
    pub fn mysql_init(connection: *mut mysql) -> *mut mysql;
    pub fn mysql_real_connect(
        connection: *mut mysql,
        server_host: *const c_char,
        sql_user_name: *const c_char,
        sql_password: *const c_char,
        db_name: *const c_char,
        port_number: c_uint,
        unix_socket_name: *const c_char,
        flags: c_uint,
    ) -> *mut mysql;
    pub fn mysql_close(connection: *mut mysql);
    /// return 0 if ping success
    pub fn mysql_ping(connection: *mut mysql) -> c_int;
}

pub const MYSQL_DEFAULT_PORT: c_uint = 0;
