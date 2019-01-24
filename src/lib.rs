use std::ffi::CStr;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use libc::{c_char, uint8_t, uint16_t, uint64_t};
use byteorder::{ByteOrder, NetworkEndian};


// These are here for compatibility with the SDL version.
#[no_mangle]
pub extern fn messend_startup() {
}
#[no_mangle]
pub extern fn messend_shutdown() {
}

#[no_mangle]
pub extern fn messend_acceptor_create(host: *const c_char, port: uint16_t) -> *mut Acceptor {
    unsafe {
        let host = CStr::from_ptr(host).to_owned().into_string().unwrap();
        let addr = format!("{}:{}", host, port);
        Box::into_raw(Box::new(Acceptor::new(addr)))
    }
}

#[no_mangle]
pub extern fn messend_acceptor_free(ptr: *mut Acceptor) {
    if ptr.is_null() {
        return
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern fn messend_acceptor_accept_wait(ptr: *mut Acceptor) -> *mut Peer {
    let acceptor = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    Box::into_raw(Box::new(acceptor.accept_wait()))
}

//#[no_mangle]
//pub extern fn peer_send_message(mut peer: Peer, message: *const u8, size: u64) {
//
//    let mut out = Vec::new();
//
//    unsafe {
//        for i in 0..size {
//            let byte = *message.offset(i as isize);
//            out.push(byte)
//        }
//    }
//
//    peer.send_message(&out)
//}
//
#[no_mangle]
pub extern fn messend_peer_receive_message_wait(ptr: *mut Peer) -> CMessage {
    let peer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let message = peer.receive_message_wait();
    //println!("{:?}", message);

    //let mut buf = vec![0; 512].into_boxed_slice();
    let mut buf = message.into_boxed_slice();
    let data = buf.as_mut_ptr();
    let len = buf.len();
    std::mem::forget(buf);

    CMessage {
        data,
        size: len as u64,
    }
}

#[no_mangle]
pub extern fn messend_peer_send_message(ptr: *mut Peer, message: CMessage) {
    let s = unsafe { 
        std::slice::from_raw_parts_mut(message.data, message.size as usize)
    };

    let peer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    println!("{:?}", s);
    peer.send_message(s);
}

#[no_mangle]
pub extern fn messend_message_free(message: CMessage) {
    let s = unsafe { 
        std::slice::from_raw_parts_mut(message.data, message.size as usize)
    };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
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

    pub fn accept_wait(&self) -> Peer {

        let stream = self.listener.accept().unwrap().0;
        Peer::new(stream)
    }
}
//
//pub struct Initiator {
//}
//
//impl Initiator {
//    // TODO: return a Result type
//    pub fn new() -> Initiator {
//
//        Initiator {
//        }
//    }
//
//    pub fn initiate<A: ToSocketAddrs>(&self, addr: A) -> Peer {
//        let stream = TcpStream::connect(addr).unwrap();
//        Peer::new(stream)
//    }
//}
//
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
        let mut buf = [0; 4];
        NetworkEndian::write_u32(&mut buf, message.len() as u32);
        self.stream.write(&buf).unwrap();
        self.stream.write(message).unwrap();
    }

    pub fn receive_message_wait(&mut self) -> Vec<u8> {
        let mut buf = [0; 4];

        self.stream.read_exact(&mut buf).unwrap();

        let size = NetworkEndian::read_u32(&buf);
        //println!("{}", size);

        let mut vec = vec![0; size as usize];
        self.stream.read_exact(&mut vec).unwrap();

        vec
    }
}

#[repr(C)]
pub struct CMessage {
    data: *mut uint8_t,
    size: uint64_t,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
