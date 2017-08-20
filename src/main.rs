//#![feature(unique)]
extern crate libc;
extern crate rpi_ws281x;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use rustc_serialize::json;
//use std::ptr::Unique;

const INDEX_HTML: &'static [u8] = include_bytes!("static/index.html");

fn main() {
    let strip = Arc::new(Mutex::new(rpi_ws281x::WS281x::new(21, 300, rpi_ws281x::StripType::WS2811StripGRB).unwrap()));
    let strip_clone = strip.clone();
    //let strip = Arc::new(Mutex::new(Unique(rpi_ws281x::WS281x::new(21, 288, rpi_ws281x::StripType::WS2811StripGRB).unwrap())));

    let mut router = Router::new();

    router.get("/", webpage, "index");

    router.post("/set_form", move |r: &mut Request| set_form(r, &mut strip.lock().unwrap()), "set_form");

    router.post("/set", move |r: &mut Request| set_strip(r, &mut strip_clone.lock().unwrap()), "set");

    fn webpage(_: &mut Request) -> IronResult<Response> {
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, INDEX_HTML)))
    }

    fn set_form(request: &mut Request, strip: &mut rpi_ws281x::WS281x) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();

        let r: u8;
        let g: u8;
        let b: u8;

        let split = payload.split("&").collect::<Vec<&str>>();
        r = split[0].to_string().split_off(2).parse().unwrap();
        g = split[1].to_string().split_off(2).parse().unwrap();
        b = split[2].to_string().split_off(2).parse().unwrap();

        println!("{:?}", payload);
        strip.display_color(r, g, b);
        Ok(Response::with(status::Ok))
    }

    fn set_strip(request: &mut Request, strip: &mut rpi_ws281x::WS281x) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let c: (u8, u8, u8) = json::decode(&payload).unwrap();
        strip.display_color(c.0, c.1, c.2);
        Ok(Response::with(status::Ok))
    }

    Iron::new(router).http("0.0.0.0:3000").unwrap();
}
