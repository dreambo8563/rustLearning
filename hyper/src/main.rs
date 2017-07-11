extern crate hyper;
extern crate futures;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use std::ascii::AsciiExt;
use futures::Stream;
use hyper::Chunk;
use hyper::Body;
use futures::future::{Either, FutureResult};
use futures::stream::Concat;
use futures::Future;
// #[derive(Debug)]
// struct HelloWorld;

// const PHRASE: &'static str = "Hello, world";

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}

// impl Service for HelloWorld {
//     // add code here
//     type Request = Request;
//     type Response = Response;
//     type Error = hyper::Error;

//     type Future = futures::future::FutureResult<Self::Response, Self::Error>;

//     fn call(&self, _req: Request) -> Self::Future {
//         futures::future::ok(
//             Response::new()
//                 .with_header(ContentLength(PHRASE.len() as u64))
//                 .with_body(PHRASE),
//         )
//     }
// }

struct Echo;

impl Service for Echo {
    // ... types here

    type Future = Either<
        FutureResult<Self::Response, Self::Error>,
        futures::future::Map<Concat<Body>, fn(Chunk) -> Self::Response>,
    >;

    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                Either::A(futures::future::ok(
                    Response::new().with_body("Try POSTing data to /echo"),
                ))
            }
            (&Method::Post, "/stream") => {
                Either::A(futures::future::ok(Response::new().with_body(req.body())))
            }
            (&Method::Post, "/echo") => Either::B(req.body().concat().map(reverse)),
            _ => {
                Either::A(futures::future::ok(
                    Response::new().with_status(StatusCode::NotFound),
                ))
            }
        }
    }
}

fn to_uppercase(chunk: Chunk) -> Chunk {
    let uppered = chunk
        .into_iter()
        .map(|byte| {
            println!("low:{},upper{}", byte, byte.to_ascii_uppercase());
            byte.to_ascii_uppercase()
        })
        .collect::<Vec<u8>>();
    Chunk::from(uppered)
}

fn reverse(chunk: Chunk) -> Response {
    let reversed = chunk.iter().rev().cloned().collect::<Vec<u8>>();
    Response::new().with_body(reversed)
}