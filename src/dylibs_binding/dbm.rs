use libc::{c_char, c_int};

/** dbm/gdbm: a key-value single file database facilities
## 不要链接新版gdbm库
gdbm的新版跟旧版gdbm_compat完全不兼容
旧版gdbm_compat的open叫dbm_open，新版
*Beginning Linux Programming* 的示例只需要 `gcc -lgdbm_compat` 就能编译了
*/
#[link(name = "gdbm_compat")]
extern "C" {
    pub type dbm_ptr;
    /// 注意filename参数文件名不要带后缀名，dbm会自动创建基于输入文件名.dir和.pag后缀的两个文件
    #[cfg(test)]
    pub fn dbm_open(filename: *const c_char, flags: c_int, mode: libc::mode_t) -> *mut dbm_ptr;
    pub fn dbm_close(dbm_ptr: *mut dbm_ptr);
    pub fn dbm_store(
        dbm_ptr: *mut dbm_ptr,
        key_datum: datum,
        value_datum: datum,
        store_mode: libc::c_int,
    ) -> c_int;
    pub fn dbm_fetch(dbm_ptr: *mut dbm_ptr, key_datum: datum) -> datum;
    pub fn dbm_firstkey(dbm_ptr: *mut dbm_ptr) -> datum;
    pub fn dbm_nextkey(dbm_ptr: *mut dbm_ptr) -> datum;
    #[cfg(test)]
    pub fn dbm_delete(dbm_ptr: *mut dbm_ptr, key_datum: datum) -> c_int;
    //fn dbm_error(dbm_ptr: *mut dbm_ptr) -> c_int;
    //fn dbm_clearerror(dbm_ptr: *mut dbm_ptr) -> c_int;
}

/// store_mode arg of dbm_store
pub struct StoreMode;

impl StoreMode {
    pub const DBM_INSERT: c_int = 0;
    pub const DBM_REPLACE: c_int = 1;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct datum {
    /// this is a raw pointer of bytes
    pub dptr: *mut c_char,
    pub dsize: c_int,
}
