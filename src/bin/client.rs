use std::{thread, time};


fn main() {
    let mut peer = messend::initiate("127.0.0.1:9001");

    //peer.send_message(&[1,2,3,4]);

    loop {
        peer.send_message(&[1,2,3,4]);

        let message = peer.receive_message_wait();
        if message.is_none() {
            break;
        }
        println!("{:?}", message);

        thread::sleep(time::Duration::from_millis(100));
    }
}
