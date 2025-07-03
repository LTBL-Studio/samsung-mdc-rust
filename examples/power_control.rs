use std::{thread, time::Duration};

use samsung_mdc_rust::{DisplayControl, MDCSession};

fn main() {
    let mut session = MDCSession::new_from_tcp("10.0.151.55:1515".parse().unwrap()).expect("Failed to connect to device");

    session.display(0)
    .set_power_on()
    .expect("Failed to set power ON");

    println!("Powered on");
        
    thread::sleep(Duration::from_secs(10));

    session.display(0)
        .set_power_off()
        .expect("Failed to set power off");

    println!("Powered off");

}