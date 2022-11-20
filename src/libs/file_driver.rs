use mockall::automock;

#[automock]
pub mod file_driver {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    pub fn read_file(file_path: String) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        let mut file = File::open(file_path)?;
        file.read_to_end(&mut buf);
        Ok(buf)
    }
}
