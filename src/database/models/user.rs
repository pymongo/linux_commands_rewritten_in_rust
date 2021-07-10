pub type Username = [u8; 7];

/// 只要结构体的各个字段都是栈上内存，没有指针，就无需序列化(保证内存对齐跟C一样)也能读写进文件中
#[derive(Clone, Copy)]
#[repr(C)]
pub struct User {
    /// user_id from 0 to 9
    pub user_id: u8,
    /// string bytes without nul terminator
    pub username: Username,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("user_id", &self.user_id)
            .field("username", &unsafe {
                String::from_utf8_unchecked(self.username.to_vec())
            })
            .finish()
    }
}

impl User {
    pub const SIZE: usize = std::mem::size_of::<Self>();
    pub const LEN: usize = 10;
    pub fn new(user_id: u8) -> Self {
        assert!(Self::user_id_is_valid(user_id));
        let mut username = *b"user_00";
        username[5] = b'0' + (user_id / 10) % 10;
        username[6] = b'0' + user_id % 10;
        Self { user_id, username }
    }

    #[inline]
    #[allow(clippy::trivially_copy_pass_by_ref)] // must pass reference
    pub const fn as_ptr(&self) -> *const Self {
        self as *const Self
    }

    /**
    // these can compile, Rust think *mut is superset of *const?
    fn as_mut_ptr(&mut self) -> *const Self {
        self as *mut Self
    }
    */
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }

    #[allow(clippy::cast_possible_truncation)]
    pub const fn user_id_is_valid(user_id: u8) -> bool {
        user_id < Self::LEN as u8
    }
}

pub trait CrudUserDao {
    type Model;
    const DB_FILENAME: *const libc::c_char = "/tmp/my_db\0".as_ptr().cast();
    unsafe fn insert_sample_data(&self);
    unsafe fn select_all(&self) -> Vec<Self::Model>;
    unsafe fn find_user_by_id(&self, user_id: u8) -> Self::Model;
    unsafe fn update_username_by_id(&self, user_id: u8, username: Username);
}

#[cfg(test)]
pub fn test_user_crud<DB: CrudUserDao<Model = User>>(db_adapter: &DB) {
    unsafe {
        db_adapter.insert_sample_data();
        dbg!(db_adapter.select_all());

        assert_eq!(db_adapter.find_user_by_id(3).username, *b"user_03");
        db_adapter.update_username_by_id(3, *b"tuesday");
        assert_eq!(db_adapter.find_user_by_id(3).username, *b"tuesday");
    }
}
