extern crate debug;
extern crate azure;
extern crate http;

use std::str;
use std::io::println;
use http::client::RequestWriter;
use http::headers::HeaderEnum;
use azure::blobstorage;

fn print_request(request: &RequestWriter) {
    println!("[33;1mRequest[0m");
    println!("[33;1m=======[0m");
    println!("");
    println!("[1mURL:[0m {}", request.url.to_str());
    println!("[1mRemote address:[0m {}", request.remote_addr);
    println!("[1mMethod:[0m {}", request.method);
    println!("[1mHeaders:[0m");

    for header in request.headers.iter() {
        println!(" - {}: {}", header.header_name(), header.header_value());
    }
}

fn main() {
    // Please don't be mean...
    let client = blobstorage::new_client("camlidev", "M5tC8FlOa5zxXPKngv57BUU82cg72t67Bznoq2PDowAm/EtoU+QJn0HcJHSzkqL6Iw06ulOa2rKaG+Fzy/b1ow==");

    let mut request = client.new_list_blob_req("camlidev-test-1");
    
    // put extra headers on
    // sign it, add auth headers
    client.sign_request(&mut request);
    print_request(&request);

    println!("");
    println!("[33;1mResponse[0m");
    println!("[33;1m========[0m");
    println!("");
    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_request) => fail!("This example can progress no further with no response :-("),
    };

    println!("[1mStatus:[0m {}", response.status);
    println!("[1mHeaders:[0m");
    for header in response.headers.iter() {
        println!(" - {}: {}", header.header_name(), header.header_value());
    }
    println!("[1mBody:[0m");
    let body = match response.read_to_end() {
        Ok(body) => body,
        Err(err) => fail!("Reading response failed: {}", err),
    };
    println(str::from_utf8(body.as_slice()).expect("Uh oh, response wasn't UTF-8"));
}