use messend::Acceptor;

fn main() {
    let acceptor = Acceptor::new("127.0.0.1:9001");

    let mut peer = acceptor.accept_wait();

    loop {
        let message = peer.receive_message_wait();
        if message.is_none() {
            break;
        }

        println!("{:?}", message);
        println!("send");
        let message = message.expect("message");
        let seq_num = message[0] + 1;
        peer.send_message(&[seq_num]);
    }
}
