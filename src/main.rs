extern crate futures;
extern crate hyper;
extern crate regex;
extern crate tokio_core;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate time;

mod client;

use client::barrage_client::BarrageClient;

fn main() {
  // 获取真实房间号
  // https://api.live.bilibili.com/room/v1/Room/room_init?id=388
  let room_id = 5096;
  let mut client = BarrageClient::connect(room_id);
  client.enter_room();
}
