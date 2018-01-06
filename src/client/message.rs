pub trait Message{
    fn to_bytes()->Vec<u8>;
}

pub struct EnterRoomMessage{
    room_id:usize,
    uid:usize
}