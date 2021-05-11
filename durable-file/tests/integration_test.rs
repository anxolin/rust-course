mod common;

#[test]
fn ok_when_close() {
    let mut file = common::tmp_durable_file();
    file.close().unwrap();
}
