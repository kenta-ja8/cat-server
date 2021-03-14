use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct MessageContent {
    field: String,
    message: String,
}
impl MessageContent {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> MessageContent {
        MessageContent {
            field: field.into(),
            message: message.into(),
        }
    }
    pub fn new_only_message(message: impl Into<String>) -> MessageContent {
        MessageContent {
            field: "".to_string(),
            message: message.into(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ResBody<T> {
    pub content: T,
    pub message_list: Vec<MessageContent>,
}
