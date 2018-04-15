extern crate ratings;

extern crate gotham;
extern crate hyper;
extern crate mime;

extern crate rand;

use rand::Rng;

use gotham::test::TestServer;
use hyper::StatusCode;

#[test]
fn it_saves_ratings() {
    let random_name = rand::thread_rng()
        .gen_ascii_chars()
        .take(20)
        .collect::<String>();
    let random_rating = rand::thread_rng().gen::<u8>();

    let test_server = TestServer::new(ratings::router()).unwrap();
    let response_post = test_server
        .client()
        .post(
            "http://localhost/ratings/5",
            format!(
                "{{\"reviewer\":\"{}\",\"rating\":{}}}",
                random_name, random_rating
            ),
            mime::APPLICATION_JSON,
        )
        .perform()
        .unwrap();

    assert_eq!(response_post.status(), StatusCode::Ok);

    let response_get = test_server
        .client()
        .get("http://localhost/ratings/5")
        .perform()
        .unwrap();

    assert_eq!(response_get.status(), StatusCode::Ok);

    assert_eq!(
        response_get.read_utf8_body().unwrap(),
        format!(
            "{{\"id\":5,\"ratings\":{{\"{}\":{}}}}}",
            random_name, random_rating
        )
    );
}
