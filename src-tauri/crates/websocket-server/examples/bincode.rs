use websocket_server::{Message, TextMessage};


fn main() {
    let message = Message::Text(TextMessage { receiver_id: Some(1), message: "test".to_string(), group_id: None });
    let serialized = bincode::serialize(&message).unwrap();
    println!("{:?}", serialized);
    let deserialized: Message = bincode::deserialize(&serialized).unwrap();
    println!("{:?}", deserialized);
}