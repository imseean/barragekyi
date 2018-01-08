//! Bilibili直播弹幕姬

extern crate futures;
extern crate hyper;
extern crate regex;
extern crate tokio_core;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate time;

pub mod client;

use client::barrage_client::BarrageClient;

fn main() {
  let room_id = 102;
  let mut client = BarrageClient::new(room_id);
  client.connect();
  loop {}
}
