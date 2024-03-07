use rocs::rpc::call_type::CallMessageType;
use rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationRequest;
use rust_ocpp::v2_0_1::messages::boot_notification::BootNotificationResponse;
use rust_ocpp::v2_0_1::datatypes::charging_station_type::ChargingStationType;
use rocs::rpc::message_type::MessageType::*;
use rocs::rpc::message_type::OcppPayload::*;
use chrono::Utc;
use uuid::Uuid;
use tokio_tungstenite::{connect_async, tungstenite::{client::IntoClientRequest, Message}};
use futures_util::{SinkExt, StreamExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut request = "ws://troca.trialog.com/ocpp-internal/ocpp201/MARTIN".into_client_request()?;
    let headers = request.headers_mut();
    headers.insert("Sec-WebSocket-Protocol", "ocpp2.0.1".parse().unwrap());

    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let boot_not_req = BootNotificationRequest {
        reason: rust_ocpp::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType::PowerUp,
        charging_station: ChargingStationType {
            model: "Martin".into(),
            vendor_name: "Litre".into(),
            firmware_version: None,
            serial_number: None,
            modem: None
        }
    };
    //let copy_req = boot_not_req.clone();
    let mut serialized = serde_json::to_string(&boot_not_req).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: BootNotificationRequest = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let boot_not_resp = BootNotificationResponse {
        current_time: Utc::now(),
        interval: 20,
        status: rust_ocpp::v2_0_1::enumerations::registration_status_enum_type::RegistrationStatusEnumType::Accepted,
        status_info: None
    };

    serialized = serde_json::to_string(&boot_not_resp).unwrap();
    println!("serialized = {}", serialized);

    let deserialized2: BootNotificationResponse = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized2);
    let call_message: CallMessageType = CallMessageType {
        message_type_id: CALL,
        message_id: Uuid::new_v4().to_string().into(),
        action: "BootNotification".into(),
        payload: BootNotificationReq201(boot_not_req)
    };
    serialized = serde_json::to_string(&call_message).unwrap();
    ws_sender.send(Message::Text(serialized.clone())).await?;
    println!("Tx >> {:?}", serialized);
    while let Some(msg) = ws_receiver.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            println!("RX << {:?}", msg);
        }
    }

    println!("serialized = {}", serialized);
    let call = r#"[2,"19223201","BootNotification",{"reason": "PowerUp","chargingStation": {"model": "SingleSocketCharger","vendorName": "VendorX"}}]"#;
    let deserialized3: CallMessageType = serde_json::from_str(&call).unwrap();
    println!("deserialized call = {:?}", deserialized3);
    println!("Payload = {:?}", &deserialized3.payload);
    println!("Message ID = {}", &deserialized3.message_id);
    println!("Message Type = {:?}", &deserialized3.message_type_id);
    serialized = serde_json::to_string(&deserialized3).unwrap();
    println!("ReSerialized = {:?}", serialized);
    println!("ReSerialized = {:?}", serde_json::to_string(&deserialized3));
    Ok(())
}
