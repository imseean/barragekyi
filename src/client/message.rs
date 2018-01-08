use serde_json;
use std::mem::transmute;

pub trait Message {
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnterRoomMessage {
    #[serde(rename = "roomid")] pub room_id: usize,
    #[serde(rename = "uid")] pub uid: usize,
}
impl EnterRoomMessage {
    pub fn new(room_id: usize) -> EnterRoomMessage {
        return EnterRoomMessage {
            room_id: room_id,
            uid: 155973685728160,
        };
    }
}

impl Message for EnterRoomMessage {
    fn to_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        let json_text = serde_json::to_string(self).unwrap();
        let mut json_data = json_text.into_bytes();
        // 数据长度
        let length: u32 = (json_data.len() as u32) + 16;
        let bytes: [u8; 4] = unsafe { transmute(length.to_be()) };
        data.append(&mut bytes.to_vec());
        // 不知道是啥
        data.append(&mut [0x00, 0x10, 0x00, 0x01].to_vec());
        // 0x7 进入房间
        data.append(&mut [0x00, 0x00, 0x00, 0x07].to_vec());
        // 不知道是啥
        data.append(&mut [0x00, 0x00, 0x00, 0x01].to_vec());

        data.append(&mut json_data);

        print!("[");
        for byte in &data {
            print!("{:0>2x} ", byte);
        }
        print!("]\r\n");
        return data;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeatMessage {
    #[serde(rename = "roomid")] pub room_id: usize,
    #[serde(rename = "uid")] pub uid: usize,
}
impl BeatMessage {
    pub fn new(room_id: usize) -> BeatMessage {
        return BeatMessage {
            room_id: room_id,
            uid: 0,
        };
    }
}

impl Message for BeatMessage {
    fn to_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        let json_text = serde_json::to_string(self).unwrap();
        let mut json_data = json_text.into_bytes();
        // 数据长度
        let length: u32 = (json_data.len() as u32) + 16;
        let bytes: [u8; 4] = unsafe { transmute(length.to_be()) };
        data.append(&mut bytes.to_vec());
        // 不知道是啥
        data.append(&mut [0x00, 0x10, 0x00, 0x01].to_vec());
        // 0x2 心跳
        data.append(&mut [0x00, 0x00, 0x00, 0x02].to_vec());
        // 不知道是啥
        data.append(&mut [0x00, 0x00, 0x00, 0x01].to_vec());

        data.append(&mut json_data);

        print!("[");
        for byte in &data {
            print!("{:0>2x} ", byte);
        }
        print!("]\r\n");
        return data;
    }
}
