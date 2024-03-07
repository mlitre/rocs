#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    CALL = 2,
    CALLRESULT = 3,
    CALLERROR = 4
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum OcppPayload {
    BootNotificationReq201(rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationRequest),
    BootNotificationRes201(rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationResponse),

    BootNotificationReq16(rust_ocpp::v1_6::messages::boot_notification::BootNotificationRequest),
    BootNotificationRes16(rust_ocpp::v1_6::messages::boot_notification::BootNotificationResponse),
}
