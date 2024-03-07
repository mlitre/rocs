use super::message_type::{MessageType, OcppPayload};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple, Debug)]
pub struct CallMessageType {
    pub message_type_id: MessageType,
    pub message_id: String,
    pub action: String,
    pub payload: OcppPayload
}
