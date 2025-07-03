use std::{thread, time::Duration};

use samsung_mdc_rust::{DisplayControl, MDCSession};

fn main() {
    let mut session = MDCSession::new_from_tcp("10.0.151.55:1515".parse().unwrap()).expect("Failed to connect to device");

    loop {
        session.display(0)
            .set_panel_on()
            .expect("Failed to set panel ON");
        
        println!("ON");
            
        thread::sleep(Duration::from_secs(5));

        session.display(0)
            .set_panel_off()
            .expect("Failed to set panel off");

        println!("OFF");

        thread::sleep(Duration::from_secs(5));
    }
}