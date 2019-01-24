#include <stdio.h>
#include <stdint.h>
#include "messend.h"


int main(int argc, char **argv) {
    MessendAcceptor acceptor = messend_acceptor_create("127.0.0.1", 9001);
    MessendPeer peer = messend_acceptor_accept_wait(acceptor);

    MessendMessage message = messend_peer_receive_message_wait(peer);

    for (int i = 0; i < message.size; i++) {
        printf("%d ", message.data[i]);
    }
    printf("\n");

    messend_peer_send_message(peer, message);
    messend_message_free(message);

    //messend_peer_send_message(peer, (Message)"Hi there", 8);
    ////Message msg = peer_receive_message(peer);

    messend_acceptor_free(acceptor);

    return 0;
}
