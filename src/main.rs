extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate regex;

mod client;

use client::barrage_client::BarrageClient;

fn main() {
  let room_id = 422332;
  let client = BarrageClient::connect(room_id);
}
