use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use regex::Regex;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct BarrageClient {
    room_id: usize,
    tcp_client:TcpStream,
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

    pub fn connect(room_id: usize) -> BarrageClient {
        let server_address = BarrageClient::get_barrage_address(room_id);
        let port = 788;

        let stream = TcpStream::connect(format!("{}:{}", server_address, port)).unwrap();
        BarrageClient {
            room_id: room_id,
            tcp_client:stream,
        }
    }
}
