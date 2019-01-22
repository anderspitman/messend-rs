#include <stdio.h>
#include <stdint.h>

typedef char* Address;
typedef void* Acceptor;
typedef void* Peer;
typedef uint8_t* Message;

extern char* hello_rust();
extern Acceptor create_acceptor(Address addr);
extern Peer acceptor_accept();
extern void peer_send_message(Peer peer, Message message, uint64_t size);
extern Message peer_receive_message(Peer peer);

int main(int argc, char **argv) {
        Acceptor acceptor = create_acceptor("127.0.0.1:9001");
        printf("%p\n", acceptor);
        Peer peer = acceptor_accept(acceptor);

        peer_send_message(peer, (Message)"Hi there", 8);
        //Message msg = peer_receive_message(peer);

        return 0;
}
