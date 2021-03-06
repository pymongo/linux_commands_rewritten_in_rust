use crate::database::models::user::{CrudUserDao, User, Username};
use crate::syscall;

struct MmapDb {
    mapped_addr: *mut libc::c_void,
}

impl MmapDb {
    const MAPPED_BYTES: usize =
        <Self as CrudUserDao>::Model::LEN * <Self as CrudUserDao>::Model::SIZE;
    #[cfg(test)]
    fn new() -> Self {
        // let fd = syscall!(open(Self::DB_FILENAME, libc::O_RDWR | libc::O_CREAT, libc::S_IRUSR | libc::S_IWUSR,));
        // insert bytes to file to fit the mapped_len required
        // syscall!(write(fd, [0_u8; Self::MAPPED_BYTES].as_ptr().cast(), Self::MAPPED_BYTES));

        let mapped_addr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                Self::MAPPED_BYTES,
                libc::PROT_READ | libc::PROT_WRITE,
                // MAP_SHARED: The segment changes are made in the file
                // MAP_ANONYMOUS: not map from file, would init region to zero and ignore fd and offset arg
                libc::MAP_SHARED | libc::MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        if mapped_addr == libc::MAP_FAILED {
            // mmap return 0 is ok, !0 is libc::MAP_FAILED
            panic!("{}", std::io::Error::last_os_error());
        }
        // mmap成功后就可以关闭fd，关闭fd不会影响mmap
        // syscall!(close(fd));
        Self { mapped_addr }
    }
}

impl Drop for MmapDb {
    fn drop(&mut self) {
        syscall!(munmap(self.mapped_addr, Self::MAPPED_BYTES));
    }
}

impl CrudUserDao for MmapDb {
    type Model = User;
    unsafe fn insert_sample_data(&self) {
        // 注意不能解引用，否则解引用之后会是Copy语义，不能修改到mmap对应的文件数据
        let users = self.mapped_addr.cast::<[Self::Model; Self::Model::LEN]>();
        for user_id in 0..Self::Model::LEN {
            let user = Self::Model::new(user_id as u8);
            (*users)[user_id] = user;
        }
        // optional if single process: sync mmap change to files
        libc::msync(self.mapped_addr, Self::MAPPED_BYTES, libc::MS_SYNC);
    }

    unsafe fn select_all(&self) -> Vec<Self::Model> {
        let users = *self.mapped_addr.cast::<[Self::Model; Self::Model::LEN]>();
        users.to_vec()
    }

    unsafe fn find_user_by_id(&self, user_id: u8) -> Self::Model {
        assert!(User::user_id_is_valid(user_id));
        let users = *self.mapped_addr.cast::<[Self::Model; Self::Model::LEN]>();
        users[usize::from(user_id)]
    }

    unsafe fn update_username_by_id(&self, user_id: u8, username: Username) {
        assert!(User::user_id_is_valid(user_id));
        let users = self.mapped_addr.cast::<[Self::Model; Self::Model::LEN]>();
        (*users)[usize::from(user_id)].username = username;
    }
}

#[test]
fn test_mmap_database() {
    let db_adapter = MmapDb::new();
    crate::database::models::user::test_user_crud(&db_adapter);
}
