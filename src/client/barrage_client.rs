//! 弹幕客户端

use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use regex::Regex;
use std::io::prelude::*;
use std::net::TcpStream;
use super::message::*;
use super::model::*;
use time;
use time::Tm;

use std::thread;
use std::sync::mpsc::channel;
use std::mem;
use std;

use std::sync::{Arc, Mutex};

fn print_bytes(data: &Vec<u8>) {
    print!("[");
    for byte in data {
        print!("{:0>2x} ", byte);
    }
    print!("]\r\n");
}

pub struct BarrageClient {
    room_id: usize,
    barrage_address: String,
    stream: Option<TcpStream>,
    last_beat_time: Tm,
}

impl BarrageClient {
    fn print_hello(&self) {
        println!("hello");
    }

    /// 创建弹幕客户端
    ///
    /// 传入直播间号，将会自动获取真实房间号。
    pub fn new(room_id: usize) -> BarrageClient {
        let room_id = BarrageClient::get_real_room_id(room_id);
        let barrage_address = BarrageClient::get_barrage_address(room_id);

        BarrageClient {
            room_id: room_id,
            barrage_address: barrage_address,
            stream: None,
            last_beat_time: time::now(),
        }
    }

    /// 连接弹幕服务器，并开始接收弹幕消息。
    pub fn connect(&mut self) {
        let port = 788;
        let stream = TcpStream::connect(format!("{}:{}", self.barrage_address, port)).unwrap();
        self.stream = Some(stream);
        self.enter_room();
        self.start_receive_message();
    }

    /// 获取真实房间号
    fn get_real_room_id(room_id: usize) -> usize {
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());

        let uri = format!(
            "http://api.live.bilibili.com/room/v1/Room/room_init?id={}",
            room_id
        ).parse()
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
        let real_room_id = RoomInfoWraper::from_str(&response_text).room_info.room_id;
        return real_room_id;
    }

    /// 获取直播地址
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

    fn start_receive_message(&mut self) {
        let mut client = self.stream.as_ref().unwrap().try_clone().unwrap();
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
                // 如果data包含4个以上的元素（因为报文使用开始的4个字节表示报文长度），则尝试取出报文。
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
                        sender.send(message).unwrap();
                    } else {
                        break;
                    }
                }
            }
        });
        let mut message = receiver.recv().unwrap();
        loop {
            let mut message = receiver.recv().unwrap();
            self.message_dispatch(&mut message);
        }
    }
    /// 消息分发
    ///
    /// 通过判断消息类型，选择不同的处理方式
    fn message_dispatch(&mut self, data: &mut Vec<u8>) {
        if data.len() < 16 {
            return;
        } else {
            let header: Vec<u8> = data.drain(..16).collect();
            let message_type = unsafe {
                mem::transmute::<[u8; 4], u32>([header[11], header[10], header[9], header[8]])
            };

            if message_type == 0x5 {
                self.barrage_message_process(data.to_vec());
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
    /// 弹幕消息处理
    fn barrage_message_process(&self, message: Vec<u8>) {
        let content = String::from_utf8(message).unwrap();
        if Barrage::is_general_barrage(&content) {
            let barrage = GeneralBarrage::from_str(&content);
            println!("{}:{}", barrage.user, barrage.text);
        } else if Barrage::is_gift_barrage(&content) {
            let barrage = GiftBarrage::from_str(&content);
            println!("{}:{}x{}", barrage.user, barrage.gift_name, barrage.number);
        }
    }

    /// 发送进入房间请求。
    ///
    /// 成功后服务器会返回：
    ///
    /// ```
    /// [00 00 00 10 00 10 00 01 00 00 00 08 00 00 00 01 ]
    /// ```
    fn enter_room(&mut self) {
        let message = EnterRoomMessage::new(self.room_id);
        self.send_message(&message);
    }

    /// 发送心跳请求。
    ///
    /// 服务器超过一定时间没收到心跳，将会断开连接。
    ///
    /// 成功后服务器会返回：
    ///
    /// ```
    /// [00 00 00 27 00 10 00 01 00 00 00 02 00 00 00 01 7b 22 72 6f 6f 6d 69 64 22 3a 35 30 39 36 2c 22 75 69 64 22 3a 30 7d ]
    /// ```
    ///
    /// 返回的数据中包含在线人数。
    fn send_beat(&mut self) {
        let message = BeatMessage::new(self.room_id);
        self.send_message(&message);
        //self.recv_message();
    }

    /// 发送消息
    fn send_message(&mut self, message: &Message) {
        self.stream
            .as_ref()
            .unwrap()
            .write(&message.to_bytes().as_slice())
            .unwrap();
    }
}
