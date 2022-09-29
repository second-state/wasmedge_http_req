use http_req_wasi::request;

fn main() {
    let res = request::head("http://httpbin.org/head").unwrap();

    println!("Status: {} {}", res.status_code(), res.reason());
    println!("{:?}", res.headers());
}
