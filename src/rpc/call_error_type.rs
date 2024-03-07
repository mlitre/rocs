pub struct CallResultMessageType {
    pub message_type_id: i64,
    pub message_id: String,
    pub error_code: String,
    pub error_description: String,
    // Error Details is in reality a JSON
    pub error_details: String
}