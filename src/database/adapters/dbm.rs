use crate::database::models::user::{CrudUserDao, User, Username};
use crate::dylibs_binding::gdbm_compat::{
    datum, dbm_close, dbm_fetch, dbm_firstkey, dbm_nextkey, dbm_open, dbm_ptr, dbm_store, StoreMode,
};

pub struct DbmDb {
    dbm_ptr: *mut dbm_ptr,
}

impl Default for DbmDb {
    fn default() -> Self {
        let dbm_ptr = unsafe {
            dbm_open(
                Self::DB_FILENAME,
                libc::O_RDWR | libc::O_CREAT,
                libc::S_IRUSR | libc::S_IWUSR,
            )
        };
        if dbm_ptr.is_null() {
            panic!("{}", std::io::Error::last_os_error());
        }
        Self { dbm_ptr }
    }
}

impl Drop for DbmDb {
    #[allow(clippy::shadow_unrelated)]
    fn drop(&mut self) {
        unsafe {
            dbm_close(self.dbm_ptr);
            // python: os.path.basename, Rust: file_stem
            let file_stem = libc::strdup(Self::DB_FILENAME);
            let file_dir = libc::strcat(file_stem, ".dir\0".as_ptr().cast());
            let file_stem = libc::strdup(Self::DB_FILENAME);
            let file_pag = libc::strcat(file_stem, ".pag\0".as_ptr().cast());
            libc::unlink(file_dir);
            libc::unlink(file_pag);
        }
    }
}

impl CrudUserDao for DbmDb {
    type Model = User;
    unsafe fn insert_sample_data(&self) {
        for user_id in 0..Self::Model::LEN {
            let mut user_id = user_id as u8;
            let mut user = Self::Model::new(user_id);
            let key_datum = datum {
                dptr: (&mut user_id as *mut u8).cast(),
                dsize: std::mem::size_of_val(&user_id) as i32,
            };
            let value_datum = datum {
                dptr: user.as_mut_ptr().cast(),
                dsize: User::SIZE as i32,
            };
            assert_eq!(
                dbm_store(self.dbm_ptr, key_datum, value_datum, StoreMode::DBM_INSERT),
                0
            );
        }
    }

    unsafe fn select_all(&self) -> Vec<Self::Model> {
        let mut users = vec![];
        let mut key_datum = dbm_firstkey(self.dbm_ptr);
        loop {
            if key_datum.dptr.is_null() {
                break;
            }

            let value_datum = dbm_fetch(self.dbm_ptr, key_datum);
            if !value_datum.dptr.is_null() {
                let mut user = std::mem::zeroed::<User>();
                std::ptr::copy(
                    value_datum.dptr.cast(),
                    user.as_mut_ptr(),
                    value_datum.dsize as usize,
                );
                users.push(user);
            }

            key_datum = dbm_nextkey(self.dbm_ptr);
        }
        // 就像HashMap一样，此时的users是无序的
        users
    }

    unsafe fn find_user_by_id(&self, mut user_id: u8) -> Self::Model {
        let key_datum = datum {
            dptr: (&mut user_id as *mut u8).cast(),
            dsize: std::mem::size_of_val(&user_id) as i32,
        };
        let value_datum = dbm_fetch(self.dbm_ptr, key_datum);
        if value_datum.dptr.is_null() {
            panic!("user_id={} not found!", user_id);
        }
        let mut user = std::mem::zeroed::<User>();
        std::ptr::copy(
            value_datum.dptr.cast(),
            user.as_mut_ptr(),
            value_datum.dsize as usize,
        );
        user
    }

    unsafe fn update_username_by_id(&self, mut user_id: u8, username: Username) {
        let key_datum = datum {
            dptr: (&mut user_id as *mut u8).cast(),
            dsize: std::mem::size_of_val(&user_id) as i32,
        };
        let mut value_datum = dbm_fetch(self.dbm_ptr, key_datum);
        if value_datum.dptr.is_null() {
            panic!("user_id={} not found!", user_id);
        }
        let mut user = std::mem::zeroed::<User>();
        std::ptr::copy(
            value_datum.dptr.cast(),
            user.as_mut_ptr(),
            value_datum.dsize as usize,
        );
        user.username = username;
        value_datum.dptr = user.as_mut_ptr().cast();
        assert_eq!(
            dbm_store(self.dbm_ptr, key_datum, value_datum, StoreMode::DBM_REPLACE),
            0
        );
    }
}

#[test]
fn test_dbm_database() {
    let db_adapter = DbmDb::default();
    crate::database::models::user::test_user_crud(&db_adapter);
}

#[cfg(test)]
unsafe fn dbm_create_read_update_delete() {
    use crate::dylibs_binding::gdbm_compat::dbm_delete;
    let handle = DbmDb::default();

    let mut key = 1;
    let mut user = User::new(key);
    let key_datum = datum {
        dptr: (&mut key as *mut u8).cast(),
        dsize: std::mem::size_of_val(&key) as i32,
    };

    // § create
    let value_datum = datum {
        dptr: user.as_mut_ptr().cast(),
        dsize: User::SIZE as i32,
    };
    assert_eq!(
        dbm_store(
            handle.dbm_ptr,
            key_datum,
            value_datum,
            StoreMode::DBM_INSERT
        ),
        0
    );

    // § read
    let mut value_datum = dbm_fetch(handle.dbm_ptr, key_datum);
    assert!(!value_datum.dptr.is_null());
    // copy user data from database
    let mut user = std::mem::zeroed::<User>();
    //std::ptr::copy(value_datum.dptr.cast(), user.as_mut_ptr(), value_datum.dsize as usize);
    libc::memcpy(
        user.as_mut_ptr().cast(),
        value_datum.dptr.cast(),
        value_datum.dsize as usize,
    );
    dbg!(user);

    // § update
    user.username = *b"tuseday";
    value_datum.dptr = user.as_mut_ptr().cast();
    assert_eq!(
        dbm_store(
            handle.dbm_ptr,
            key_datum,
            value_datum,
            StoreMode::DBM_REPLACE
        ),
        0
    );

    // § delete
    let delete_ret = dbm_delete(handle.dbm_ptr, key_datum);
    if delete_ret == 0 {
        println!("user_id={} delete success", user.user_id);
    } else {
        println!("user_id={} not exist!", user.user_id);
    }
}

#[test]
fn test_dbm_create_read_update_delete() {
    unsafe {
        dbm_create_read_update_delete();
    }
}
