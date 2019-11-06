pub trait Storage {
    fn read(&self);

    fn write(self, data: &[u8]);
}

fn touch(name: &str, create_dirs: &str) {
    if create_dirs.len() > 0 {
        std::fs::create_dir_all(create_dirs).unwrap_or(());
    }
}
