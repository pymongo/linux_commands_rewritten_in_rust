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
    /// this is a struct, not a opaque type
    pub type mysql;
    /// this is a struct, not a opaque type
    pub type mysql_res;
    /// mysql function return 0 or not_null means no error
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
    /// query arg with no terminating semicolon, query SQL statement's line break is `\`
    pub fn mysql_query(connection: *mut mysql, query: *const c_char) -> c_int;
    /// returns the number of rows affected by the UPDATE, INSERT, or DELETE query
    pub fn mysql_affected_rows(connection: *mut mysql) -> my_ulonglong;
    /// fetch all rows at a time, used in your query data total size is small(network may not incomplete data)
    pub fn mysql_store_result(connection: *mut mysql) -> *mut mysql_res;
    /// get row one by one at a time, used in return data very large(large than memory limits)
    /// use_result include the header row while store result not
    pub fn mysql_use_result(connection: *mut mysql) -> *mut mysql_res;
    pub fn mysql_free_result(res_ptr: *mut mysql_res);

    pub fn mysql_num_rows(res_ptr: *mut mysql_res) -> my_ulonglong;
    /// you must mysql_fetch_row repeatedly until all the data has been retrieved
    /// If not, subsequent operations in process to retrieve data may corrupt
    pub fn mysql_fetch_row(res_ptr: *mut mysql_res) -> MysqlRow;
    /// unsigned int STDCALL mysql_field_count(MYSQL *mysql);
    pub fn mysql_field_count(connection: *mut mysql) -> libc::c_ulong;
    // pub fn mysql_real_escape_string_quote()
}

#[allow(non_camel_case_types)]
pub type my_ulonglong = libc::c_ulonglong;
pub type MysqlRow = *mut *mut libc::c_char;

pub const MYSQL_DEFAULT_PORT: c_uint = 0;
