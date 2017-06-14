#![deny(warnings)]

extern crate hyper;
extern crate solicit;
//extern crate kinglet;
extern crate futures;
extern crate iron;
extern crate nickel;
extern crate tiny_http;
extern crate tokio_core;
extern crate pretty_env_logger;

//use std::any::{Any};

//use nickel::Nickel;
//fn nickel_example(address: String) {
//  let mut server = Nickel::new();
//
//  server.utilize(router! {
//    get "**" => |req, res| {
//      "Hello, World!\n"
//    }
//  });
//
//  server.listen(address);
//}


use std::env;
use std::io::{Read};
use hyper::Client;
fn hyper_example() {
    pretty_env_logger::init().unwrap();

    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return;
        }
    };

    let url = url.parse::<hyper::Url>().unwrap();
    let client = Client::new();

    let mut response = match client.get(url).send() {
        Ok(result) => result,
        Err(e) => {
            println!("An error occurred while trying to contact the url: {}", e);
            return;
        }
    };

    println!("Response: {}", response.status);
    println!("Headers: \n");
    for header in response.headers.iter() {
        println!("{}", header);
    }

    let mut buffer = String::new();
    match response.read_to_string(&mut buffer) {
        Ok(_) => println!("Success!"),
        Err(_) => {
            println!("An error occurred while parsing headers");
            return;
        },
    }
    println!("{}", buffer);
}


//use iron::prelude::{Request, Response, IronResult, Iron};
//use iron::status;
//fn iron_example(address: String) {
//    fn hello_world(req: &mut Request) -> IronResult<Response> {
//        println!("Got request: {} {}", req.method, req.url);
//        Ok(Response::with((status::Ok, "Hello, World!\n")))
//    }
//
//    Iron::new(hello_world).http(address).unwrap();
//}


//use tiny_http::{Server, Response};
//fn tiny_http_example(address: String) {
//    let server = Server::http(address).unwrap();
//
//    for request in server.incoming_requests() {
//        println!("Received request: {:?} {:?}, {:?}", request.method(), request.url(), request.headers());
//
//        let response = Response::from_string("Hello, World!\n");
//        match request.respond(response) {
//            Ok(_)   => println!("Success!"),
//            Err(res)  => println!("Error: {}", res),
//        }
//    }
//}


//use solicit::http::client::CleartextConnector;
//use solicit::client::SimpleClient;
//use std::str;
//fn solicit_example() {
//    let connector = CleartextConnector::new("http2bin.org");
//    let mut client = SimpleClient::with_connector(connector).unwrap();
//    let response = client.get(b"/get", &[]).unwrap();
//
//    assert_eq!(response.stream_id, 1);
//    assert_eq!(response.status_code().unwrap(), 200);
//
//    for header in response.headers.iter() {
////        println!("{}", str::from_utf8(header).unwrap());
//        let &(ref name, ref value) = header;
//        println!("{}: {}",
//             str::from_utf8(&name).unwrap(),
//             str::from_utf8(&value).unwrap(),
//        );
////        println!("{}: {}",
////             str::from_utf8(header.name()).unwrap(),
////             str::from_utf8(header.value()).unwrap()
////        );
//    }
//
//    println!("{}", str::from_utf8(&response.body).unwrap());
//
//    let req_id1 = client.request(b"GET", b"/get?hi=hello", &[], None).unwrap();
//    let req_id2 = client.request(b"GET", b"/asdf", &[], None).unwrap();
//
//    let (res1, res2) = (
//        client.get_response(req_id1).unwrap(),
//        client.get_response(req_id2).unwrap(),
//    );
//
//    assert_eq!(res1.status_code().unwrap(), 200);
//    assert_eq!(res2.status_code().unwrap(), 404);
//}


fn main() {
//    let port = 3000;
//    let address = "127.0.0.1";
//    let address_string = format!("{}:{}", address, port);

//    nickel_example(addressString);
//    iron_example(address_string);
//    tiny_http_example(address_string);
//    solicit_example();
    hyper_example();

//    println!("Listening on port {}...", port);
}



#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  }
}
