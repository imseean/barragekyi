use serde_json;
use std::mem::transmute;
use serde_json::{Error, Value};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct BarrageData {
    #[serde(rename = "cmd")] pub command: String,
}

impl BarrageData {
    pub fn is_message_barrage(content: &str) -> bool {
        let data: BarrageData = serde_json::from_str(&content).unwrap();
        return data.command == "DANMU_MSG";
    }

    pub fn is_gift_barrage(content: &str) -> bool {
        let data: BarrageData = serde_json::from_str(&content).unwrap();
        return data.command == "SEND_GIFT";
    }
}

pub struct MessageBarrageData {
    pub user: String,
    pub text: String,
}

impl MessageBarrageData {
    pub fn from_str(content: &str) -> MessageBarrageData {
        let data: Value = serde_json::from_str(&content).unwrap();
        let info = &data.as_object().unwrap()["info"];
        let text = &info.as_array().unwrap()[1].as_str().unwrap();
        let user = &info.as_array().unwrap()[2].as_array().unwrap()[1]
            .as_str()
            .unwrap();

        println!("{}:{}", user, text);

        return MessageBarrageData {
            user: user.to_string(),
            text: text.to_string(),
        };
    }
}

pub struct GiftBarrageData {
    pub user: String,
    pub number: u64,
    pub gift_name: String,
}

impl GiftBarrageData {
    pub fn from_str(content: &str) -> GiftBarrageData {
        let data: Value = serde_json::from_str(&content).unwrap();
        //println!("{:?}", data.as_object().unwrap()["data"]);
        let gift_name = data.as_object().unwrap()["data"].as_object().unwrap()["giftName"]
            .as_str()
            .unwrap();
        let number = data.as_object().unwrap()["data"].as_object().unwrap()["num"]
            .as_u64()
            .unwrap();
        let user = data.as_object().unwrap()["data"].as_object().unwrap()["uname"]
            .as_str()
            .unwrap();

        println!("{}:{}x{}", user, gift_name, number);
        return GiftBarrageData {
            user: user.to_string(),
            gift_name: gift_name.to_string(),
            number: number,
        };
        // return GiftBarrageData {
        //     user: "user".to_string(),
        //     gift_name: "gift_name".to_string(),
        //     number: 0,
        // };
    }
}
