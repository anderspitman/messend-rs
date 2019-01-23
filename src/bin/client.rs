use std::{thread, time};
use messend::Initiator;

fn main() {
    let initiator = Initiator::new();
    let mut peer = initiator.initiate("127.0.0.1:9001");

    peer.send_message(&[1,2,3,4]);

    loop {
        let message = peer.receive_message();
        println!("{:?}", message);
        peer.send_message(&[1,2,3,4]);
        thread::sleep(time::Duration::from_millis(1000));
    }
}
