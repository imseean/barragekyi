use serde_json;
use serde_json::Value;

/// 房间信息外层结构
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomInfoWraper {
    #[serde(rename = "data")] pub room_info: RoomInfo,
}

impl RoomInfoWraper {
    pub fn from_str(content: &str) -> RoomInfoWraper {
        let model: RoomInfoWraper = serde_json::from_str(&content).unwrap();
        return model;
    }
}
/// 房间信息
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomInfo {
    #[serde(rename = "room_id")] pub room_id: usize,
}

/// 弹幕数据通用结构
#[derive(Serialize, Deserialize, Debug)]
pub struct Barrage {
    #[serde(rename = "cmd")] pub command: String,
}

impl Barrage {
    pub fn is_general_barrage(content: &str) -> bool {
        let data: Barrage = serde_json::from_str(&content).unwrap();
        return data.command == "DANMU_MSG";
    }
    pub fn is_gift_barrage(content: &str) -> bool {
        let data: Barrage = serde_json::from_str(&content).unwrap();
        return data.command == "SEND_GIFT";
    }
}

/// 文本弹幕信息
pub struct GeneralBarrage {
    pub user: String,
    pub text: String,
}

impl GeneralBarrage {
    pub fn from_str(content: &str) -> GeneralBarrage {
        let data: Value = serde_json::from_str(&content).unwrap();
        let info = &data.as_object().unwrap()["info"];
        let text = &info.as_array().unwrap()[1].as_str().unwrap();
        let user = &info.as_array().unwrap()[2].as_array().unwrap()[1]
            .as_str()
            .unwrap();

        return GeneralBarrage {
            user: user.to_string(),
            text: text.to_string(),
        };
    }
}

/// 礼物弹幕信息
pub struct GiftBarrage {
    pub user: String,
    pub number: u64,
    pub gift_name: String,
}

impl GiftBarrage {
    pub fn from_str(content: &str) -> GiftBarrage {
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

        return GiftBarrage {
            user: user.to_string(),
            gift_name: gift_name.to_string(),
            number: number,
        };
    }
}
