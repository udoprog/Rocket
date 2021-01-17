use rocket::local::blocking::Client;

#[test]
fn test_root() {
    let client = Client::tracked(super::rocket()).unwrap();
    let res = client.get("/").dispatch();

    let res_str = res.into_bytes().unwrap();
    assert!(res_str.len() > 100);
}

#[test]
fn test_async_stream() {
    let client = Client::tracked(super::rocket()).unwrap();
    let res = client.get("/async-stream").dispatch();

    let res_str = res.into_bytes().unwrap();
    assert!(res_str.len() > 100);
}
