pub fn tmp_durable_file() -> durable_file::DurableFile {
    let dir = tempdir::TempDir::new("tests").unwrap();
    let file_name = dir.path().join("foo.txt");
    let file = std::fs::File::create(file_name).unwrap();

    durable_file::DurableFile::new(file)
}
