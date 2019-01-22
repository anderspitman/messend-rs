use std::os::raw::c_char;
use std::ffi::CStr;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

#[no_mangle]
pub extern fn hello_rust() -> *const u8 {
    "Hello, world!\0".as_ptr()
}

#[no_mangle]
pub extern fn create_acceptor(addr: *const c_char) -> Acceptor {
    unsafe {
        let address = CStr::from_ptr(addr).to_owned().into_string().unwrap();
        Acceptor::new(address)
    }
}

#[no_mangle]
pub extern fn acceptor_accept(acceptor: Acceptor) -> Peer {
    acceptor.accept()
}

#[no_mangle]
pub extern fn peer_send_message(mut peer: Peer, message: *const u8, size: u64) {

    let mut out = Vec::new();

    unsafe {
        for i in 0..size {
            let byte = *message.offset(i as isize);
            out.push(byte)
        }
    }

    peer.send_message(&out)
}

#[no_mangle]
pub extern fn peer_receive_message(mut peer: Peer) -> Vec<u8> {
    [1,2,3,4].to_vec()
}

pub struct Acceptor {
    listener: TcpListener,
}

impl Acceptor {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Acceptor {
        Acceptor {
            listener: TcpListener::bind(addr).unwrap(),
        }
    }

    pub fn accept(&self) -> Peer {

        let stream = self.listener.accept().unwrap().0;

        Peer::new(stream)
    }
}

pub struct Initiator {
}

impl Initiator {
    // TODO: return a Result type
    pub fn new() -> Initiator {

        Initiator {
        }
    }

    pub fn initiate<A: ToSocketAddrs>(&self, addr: A) -> Peer {
        let stream = TcpStream::connect(addr).unwrap();
        Peer::new(stream)
    }
}

pub struct Peer {
    stream: TcpStream,
}

impl Peer {
    fn new(stream: TcpStream) -> Peer {
        Peer {
            stream,
        }
    }

    pub fn send_message(&mut self, message: &[u8]) {
        // write message size
        self.stream.write(&[message.len() as u8]);
        self.stream.write(message);
    }

    pub fn receive_message(&mut self) -> Vec<u8> {
        let mut buf = [0];
        // read message size
        self.stream.read_exact(&mut buf).unwrap();
        println!("{}", buf[0]);

        let mut vec = vec![0; buf[0] as usize];
        self.stream.read_exact(&mut vec);

        vec
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
