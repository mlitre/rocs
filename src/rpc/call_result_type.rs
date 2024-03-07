use super::message_type::{OcppPayload, MessageType};

pub struct CallResultMessageType {
    pub message_type_id: MessageType,
    pub message_id: String,
    pub payload: OcppPayload
}
