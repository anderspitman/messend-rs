use std::ffi::CStr;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use libc::{c_char, uint8_t, uint16_t, uint64_t};
use byteorder::{ByteOrder, NetworkEndian};


// C Interface

#[repr(C)]
pub struct CMessage {
    data: *mut uint8_t,
    size: uint64_t,
}

// These are here for compatibility with the SDL version.
#[no_mangle]
pub extern fn messend_startup() {
}
#[no_mangle]
pub extern fn messend_shutdown() {
}

#[no_mangle]
pub extern fn messend_acceptor_create(host: *const c_char, port: uint16_t) -> *mut Acceptor {
    let host = unsafe {
        CStr::from_ptr(host)
    };

    let host = host.to_owned().into_string().unwrap();
    let addr = format!("{}:{}", host, port);
    Box::into_raw(Box::new(Acceptor::new(addr)))
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

#[no_mangle]
pub extern fn messend_initiate(host: *const c_char, port: uint16_t) -> *mut Peer {
    let host = unsafe {
        CStr::from_ptr(host)
    };

    let host = host.to_owned().into_string().unwrap();
    let addr = format!("{}:{}", host, port);

    let stream = TcpStream::connect(addr).unwrap();
    Box::into_raw(Box::new(Peer::new(stream)))
}

#[no_mangle]
pub extern fn messend_peer_free(ptr: *mut Peer) {
    if ptr.is_null() {
        return
    }
    unsafe {
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern fn messend_peer_is_connected(ptr: *mut Peer) -> bool {
    let peer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    peer.is_connected
}

#[no_mangle]
pub extern fn messend_peer_receive_message_wait(ptr: *mut Peer) -> *const CMessage {
    let peer = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    match peer.receive_message_wait() {

        Some(message) => {
            let mut buf = message.into_boxed_slice();
            let data = buf.as_mut_ptr();
            let len = buf.len();
            std::mem::forget(buf);

            Box::into_raw(Box::new(CMessage {
                data,
                size: len as u64,
            }))
        }
        None => {
            std::ptr::null()
        }
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

    peer.send_message(s);
}

#[no_mangle]
pub extern fn messend_message_free(ptr: *mut CMessage) {

    let message = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let s = unsafe { 
        std::slice::from_raw_parts_mut(message.data, message.size as usize)
    };

    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
        Box::from_raw(ptr);
    }
}




// Native Rust

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

pub struct Peer {
    pub is_connected: bool,
    stream: TcpStream,
}

impl Peer {
    fn new(stream: TcpStream) -> Peer {
        Peer {
            is_connected: true,
            stream,
        }
    }

    pub fn send_message(&mut self, message: &[u8]) -> bool {
        let mut buf = [0; 4];
        NetworkEndian::write_u32(&mut buf, message.len() as u32);
        // TODO: flatten out this nesting
        match self.stream.write(&buf) {
            Ok(_) => {
                match self.stream.write(message) {
                    Ok(_) => true,
                    Err(_) => {
                        self.is_connected = false;
                        false
                    }
                }
            }
            Err(_) => {
                self.is_connected = false;
                false
            }
        }
    }

    // TODO: return a result
    pub fn receive_message_wait(&mut self) -> Option<Vec<u8>> {
        let mut buf = [0; 4];

        match self.stream.read_exact(&mut buf) {

            Ok(_) => {
                let size = NetworkEndian::read_u32(&buf);

                let mut vec = vec![0; size as usize];
                match self.stream.read_exact(&mut vec) {
                    Ok(_) => {
                        Some(vec)
                    }
                    Err(_) => {
                        self.is_connected = false;
                        None
                    }
                }

            }
            Err(_) => {
                self.is_connected = false;
                None
            }
        }
    }
}
