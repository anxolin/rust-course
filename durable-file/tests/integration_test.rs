mod common;

#[test]
fn ok_when_close() {
    let file = common::tmp_durable_file();
    file.close().unwrap();
}
