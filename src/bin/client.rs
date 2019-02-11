use std::{thread, time};


fn main() {
    let mut peer = messend::initiate("127.0.0.1", 9001);

    //peer.send_message(&[1,2,3,4]);
    
    let mut seq_num = 0;

    loop {
        println!("send");
        peer.send_message(&[seq_num]);

        let mut message;
        loop {
            message = peer.receive_message();
            println!("{:?}", message);
            if !message.is_none() {
                break;
            }

            thread::sleep(time::Duration::from_millis(10));
        }

        //if message.is_none() {
        //    break;
        //}
        //println!("{:?}", message);

        seq_num = message.expect("msg")[0] + 1;
        //thread::sleep(time::Duration::from_millis(10));
    }
}
