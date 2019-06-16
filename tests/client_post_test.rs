use fastcgi_client::{Address, ClientBuilder, Params};
use std::env::current_dir;

mod common;

#[test]
fn test() {
    common::setup();

    let mut client = ClientBuilder::new(Address::Tcp("127.0.0.1", 9000)).build().unwrap();
    //    let mut client = ClientBuilder::new(Address::UnixSock("/run/php/php7.1-fpm.sock")).build().unwrap();

    let document_root = current_dir().unwrap().join("tests").join("php");
    let document_root = document_root.to_str().unwrap();
    let script_name = current_dir().unwrap().join("tests").join("php").join("post.php");
    let script_name = script_name.to_str().unwrap();

    let body = b"p1=3&p2=4";
    let len = format!("{}", body.len());

    let params = Params::with_predefine()
        .set_request_method("POST")
        .set_document_root(document_root)
        .set_script_name("/post.php")
        .set_script_filename(script_name)
        .set_request_uri("/post.php?g1=1&g2=2")
        .set_query_string("g1=1&g2=2")
        .set_document_uri("/post.php")
        .set_remote_addr("127.0.0.1")
        .set_remote_port("12345")
        .set_server_addr("127.0.0.1")
        .set_server_port("80")
        .set_server_name("jmjoy-pc")
        .set_content_type("application/x-www-form-urlencoded")
        .set_content_length(&len);
    let output = client.do_request(&params, &mut &body[..]).unwrap();

    assert_eq!(
        &*output.get_stdout().unwrap_or(Default::default()),
        &b"Status: 500 Internal Server Error\r\nContent-type: text/html; charset=UTF-8\r\n\r\n1234"[..]
    );
    assert_eq!(output.get_stderr().unwrap_or(Default::default()), &b"PHP message: PHP Fatal error:  Uncaught Exception: TEST in /home/jmjoy/workspace/rust/fastcgi-client-rs/tests/php/post.php:3\nStack trace:\n#0 {main}\n  thrown in /home/jmjoy/workspace/rust/fastcgi-client-rs/tests/php/post.php on line 3\n"[..]);
}