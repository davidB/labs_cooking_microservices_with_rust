extern crate ratings;

extern crate gotham;
extern crate hyper;
extern crate mime;

extern crate rand;

use rand::Rng;

use gotham::test::TestServer;
use hyper::StatusCode;

#[test]
fn it_saves_name() {
    let random_name = rand::thread_rng()
        .gen_ascii_chars()
        .take(20)
        .collect::<String>();

    let test_server = TestServer::new(ratings::example::router()).unwrap();
    let response_post = test_server
        .client()
        .post(
            "http://localhost/hello",
            format!("{{\"say hello to\": \"{}\"}}", random_name),
            mime::APPLICATION_JSON,
        )
        .perform()
        .unwrap();

    assert_eq!(response_post.status(), StatusCode::Ok);

    let response_get = test_server
        .client()
        .get("http://localhost/hello")
        .perform()
        .unwrap();

    assert_eq!(response_get.status(), StatusCode::Ok);

    assert_eq!(
        response_get.read_utf8_body().unwrap(),
        format!(
            "{{\"interjection\":\"Hello\",\"name\":\"{}\"}}",
            random_name
        )
    );
}
