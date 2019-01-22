use messy_tcp::Acceptor;

fn main() {
    let acceptor = Acceptor::new("127.0.0.1:9001");

    let mut peer = acceptor.accept();

    loop {
        let message = peer.receive_message();
        println!("{:?}", message);
        peer.send_message(&[1,2,3,4]);
    }
}
