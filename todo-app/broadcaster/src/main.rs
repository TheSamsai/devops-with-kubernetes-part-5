fn main() {
    let web_hook_url = std::env::var("WEB_HOOK").unwrap_or(String::from("https://nats.requestcatcher.com/test"));

    let nats_url = std::env::var("NATS_URL").unwrap_or(String::from("nats://localhost:4222"));
    let nats_client = nats::connect(nats_url).expect("Failed to connect to NATS");

    let sub = nats_client.subscribe("todo").expect("Failed to subscribe to topic");

    loop {
        if let Some(msg) = sub.next() {
            let msg_string = String::from_utf8(msg.data).unwrap();

            ureq::post(&web_hook_url)
                .send_json(ureq::json!({
                    "user": "bot",
                    "message": msg_string
                })).expect("Failed to send HTTP request");
        }
    }
}
