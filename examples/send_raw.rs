use samsung_mdc_rust::{commands::{self}, proto::Packet, MDCSession, DISPLAY_BROADCAST};

fn main() {
    let mut session = MDCSession::new_from_tcp(
            "10.0.151.55:1515".parse().unwrap()
        ).expect("Failed to connect to device");

    let display_id = 0;

    session.send_packet(Packet::new(commands::PANEL_ON_OFF, display_id, vec![0]))
        .expect("Failed to send packet");

    if display_id != DISPLAY_BROADCAST {
        let response = session.recv_packet().unwrap();
        println!("Response: {:?}", response);
    }

    println!("Done")
}