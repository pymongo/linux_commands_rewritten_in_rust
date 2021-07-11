use crate::database::models::user::{CrudUserDao, User, Username};

struct FreadFwriteDb {
    /// database FILE stream pointer
    db_fp: *mut libc::FILE,
}

impl FreadFwriteDb {
    #[cfg(test)]
    fn new() -> Self {
        let fp = unsafe { libc::fopen(Self::DB_FILENAME, "w+\0".as_ptr().cast()) };
        if fp.is_null() {
            panic!("{}", std::io::Error::last_os_error());
        }
        Self { db_fp: fp }
    }
}

impl Drop for FreadFwriteDb {
    fn drop(&mut self) {
        let close_ret = unsafe { libc::fclose(self.db_fp) };
        if close_ret == -1 {
            panic!("{}", std::io::Error::last_os_error());
        }
        unsafe {
            libc::unlink(Self::DB_FILENAME);
        }
    }
}

impl CrudUserDao for FreadFwriteDb {
    type Model = User;
    /**
    ```text
    $ od -c target/users_db
    0000000  \0   u   s   e   r   _   0   0 001   u   s   e   r   _   0   1
    0000020 002   u   s   e   r   _   0   2 003   a   c   c   o   u   n   t
    0000040 004   u   s   e   r   _   0   4 005   u   s   e   r   _   0   5
    0000060 006   u   s   e   r   _   0   6  \a   u   s   e   r   _   0   7
    0000100  \b   u   s   e   r   _   0   8  \t   u   s   e   r   _   0   9
    0000120
    ```
    note that user_id=006 is escape to b'\a' in od
    */
    unsafe fn insert_sample_data(&self) {
        for user_id in 0..Self::Model::LEN {
            let user = Self::Model::new(user_id as u8);
            libc::fwrite(user.as_ptr().cast(), Self::Model::SIZE, 1, self.db_fp);
        }
    }

    unsafe fn select_all(&self) -> Vec<Self::Model> {
        let mut users = [std::mem::zeroed::<Self::Model>(); Self::Model::LEN];
        libc::fseek(self.db_fp, 0, libc::SEEK_SET);
        let read_count = libc::fread(
            users.as_mut_ptr().cast(),
            Self::Model::LEN,
            Self::Model::LEN,
            self.db_fp,
        );
        assert_ne!(read_count, 0);
        users.to_vec()
    }

    unsafe fn find_user_by_id(&self, user_id: u8) -> Self::Model {
        assert!(User::user_id_is_valid(user_id));
        let mut user = std::mem::zeroed::<Self::Model>();
        libc::fseek(
            self.db_fp,
            libc::c_long::from(user_id) * Self::Model::SIZE as libc::c_long,
            libc::SEEK_SET,
        );
        libc::fread(user.as_mut_ptr().cast(), Self::Model::SIZE, 1, self.db_fp);
        user
    }

    unsafe fn update_username_by_id(&self, user_id: u8, username: Username) {
        assert!(User::user_id_is_valid(user_id));
        let offset = libc::c_long::from(user_id) * Self::Model::SIZE as libc::c_long;
        let mut user = std::mem::zeroed::<Self::Model>();
        libc::fseek(self.db_fp, offset, libc::SEEK_SET);
        libc::fread(user.as_mut_ptr().cast(), Self::Model::SIZE, 1, self.db_fp);
        user.username = username;
        libc::fseek(self.db_fp, offset, libc::SEEK_SET); // reset cursor after fread
        libc::fwrite(user.as_ptr().cast(), Self::Model::SIZE, 1, self.db_fp);
    }
}

#[test]
fn test_stdio_database() {
    let db_adapter = FreadFwriteDb::new();
    crate::database::models::user::test_user_crud(&db_adapter);
}
