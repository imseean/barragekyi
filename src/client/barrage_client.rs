use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use regex::Regex;
use std::io::prelude::*;
use std::net::TcpStream;
use super::message::*;
use time;
use time::{now, Tm};

use std::thread;
use std::sync::mpsc::{channel, Receiver};
use std::mem;

fn print_bytes(data: &Vec<u8>) {
    print!("[");
    for byte in data {
        print!("{:0>2x} ", byte);
    }
    print!("]\r\n");
}

pub struct BarrageClient {
    room_id: usize,
    tcp_client: TcpStream,
    last_beat_time: Tm,
}

impl BarrageClient {
    fn get_barrage_address(room_id: usize) -> String {
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        let uri = format!("http://live.bilibili.com/api/player?id=cid:{}", room_id)
            .parse()
            .unwrap();
        let mut response_text = String::new();
        {
            let work = client.get(uri).and_then(|response| {
                println!("Response: {}", response.status());
                response.body().for_each(|chunk| {
                    response_text = String::from_utf8(chunk.to_vec()).unwrap();
                    Ok(())
                })
            });
            core.run(work).unwrap();
        }
        let regex = Regex::new(r"<server>(?P<server>.+)</server>").unwrap();
        let matchs = regex.captures(&response_text).unwrap();
        println!("{:?}", &matchs["server"]);
        String::from(&matchs["server"])
    }

    fn send_message(&mut self, message: &Message) {
        self.tcp_client
            .write(&message.to_bytes().as_slice())
            .unwrap();
    }

    fn recv_message(&mut self) {
        let mut client = self.tcp_client.try_clone().unwrap();
        let (sender, receiver) = channel::<Vec<u8>>();
        let handler = thread::spawn(move || {
            let mut data: Vec<u8> = Vec::new();
            let mut buffer = [0; 128];
            loop {
                let length = client.read(&mut buffer).unwrap();
                if length == 0 {
                    continue;
                }
                let mut buffer = buffer.to_vec();
                buffer.truncate(length);
                data.append(&mut buffer);
                while data.len() > 4 {
                    let length;
                    {
                        let length_data = data.get(0..4);
                        if None == length_data {
                            continue;
                        }
                        let length_data = length_data.unwrap();
                        length = unsafe {
                            mem::transmute::<[u8; 4], u32>([
                                length_data[3],
                                length_data[2],
                                length_data[1],
                                length_data[0],
                            ])
                        };
                    }

                    if length <= data.len() as u32 {
                        let message: Vec<u8> = data.drain(..(length as usize)).collect();
                        // println!("收到消息：");
                        // print_bytes(&message.to_vec());
                        sender.send(message).unwrap();
                    } else {
                        break;
                    }
                }
            }
        });
        loop {
            let mut message = receiver.recv().unwrap();
            self.message_dispatch(&mut message);
        }
        handler.join().unwrap();
    }

    fn message_dispatch(&mut self, data: &mut Vec<u8>) {
        if data.len() < 16 {
            return;
        } else {
            let header: Vec<u8> = data.drain(..16).collect();
            let message_type = unsafe {
                mem::transmute::<[u8; 4], u32>([header[11], header[10], header[9], header[8]])
            };

            if message_type == 0x5 {
                self.barrage_message_handle(data.to_vec());
            } else {
                println!("消息类型:{}", message_type);
            }

            // 发送心跳
            let now = time::now();
            let ts = now - self.last_beat_time;
            if ts.num_seconds() > 25 {
                self.send_beat();
                self.last_beat_time = now;
            }
        }
    }
    fn barrage_message_handle(&self, message: Vec<u8>) {
        let content = String::from_utf8(message).unwrap();
        if BarrageData::is_message_barrage(&content) {
            MessageBarrageData::from_str(&content);
        } else if BarrageData::is_gift_barrage(&content) {
            GiftBarrageData::from_str(&content);
        }
    }

    pub fn connect(room_id: usize) -> BarrageClient {
        let server_address = BarrageClient::get_barrage_address(room_id);
        // let server_address = "127.0.0.1";
        let port = 788;

        let stream = TcpStream::connect(format!("{}:{}", server_address, port)).unwrap();
        BarrageClient {
            room_id: room_id,
            tcp_client: stream,
            last_beat_time: time::now(),
        }
    }

    pub fn enter_room(&mut self) {
        let message = EnterRoomMessage::new(self.room_id);
        self.send_message(&message);
        self.recv_message();
    }

    pub fn send_beat(&mut self) {
        let message = BeatMessage::new(self.room_id);
        self.send_message(&message);
        //self.recv_message();
    }
}
