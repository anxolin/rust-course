use std::fs;
use std::io;

#[derive(Debug)]
pub struct DurableFile {
    pub file: fs::File,
    pub need_sync: bool,
}

impl DurableFile {
    pub fn new(file: fs::File) -> DurableFile {
        DurableFile {
            file,
            need_sync: true, // we don't know if is in sync or not. Take the worst case
        }
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.file.sync_all()?;
        self.need_sync = false;

        Ok(())
    }
}

impl io::Write for DurableFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.need_sync = true;
        self.file.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.need_sync = false;

        self.file.flush()
    }
}

impl Drop for DurableFile {
    fn drop(&mut self) {
        println!("Cleaning up! 🧹"); // Just for learning purposes

        if self.need_sync {
            panic!("You forgot to sync!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_durable_file() -> DurableFile {
        let dir = tempdir::TempDir::new("tests").unwrap();
        let file_name = dir.path().join("foo.txt");
        let file = std::fs::File::create(file_name).unwrap();

        DurableFile::new(file)
    }

    #[test]
    #[should_panic(expected = "You forgot to sync")]
    fn raise_when_left_open() {
        new_durable_file();
    }

    #[test]
    fn ok_when_close() {
        let mut file = new_durable_file();
        file.close().unwrap();
    }

    #[test]
    fn need_async_flag_true() {
        let mut file = new_durable_file();
        assert_eq!(file.need_sync, true);
        file.close().unwrap();
    }

    #[test]
    fn need_async_flag_false() {
        let mut file = new_durable_file();
        file.close().unwrap();
        assert_eq!(file.need_sync, false)
    }
}
