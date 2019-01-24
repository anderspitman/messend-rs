#include <stdio.h>
#include <stdint.h>
#include "messend.h"


int main(int argc, char **argv) {

    messend_startup();

    MessendAcceptor acceptor = messend_acceptor_create("127.0.0.1", 9001);

    printf("Waiting for connection\n");
    MessendPeer peer = messend_acceptor_accept_wait(acceptor);
    printf("Connection established\n");

    while (1) {

        MessendMessage recvMessage = messend_peer_receive_message_wait(peer);

        printf("Message received:\n");

        for (int i = 0; i < recvMessage.size; i++) {
            printf("%c", ((uint8_t*)(recvMessage.data))[i]);
        }
        printf("\n");

        messend_peer_send_message(peer, recvMessage);
        messend_message_free(recvMessage);
    }

    messend_peer_free(peer);
    peer = 0;

    messend_acceptor_free(acceptor);
    acceptor = NULL;

    messend_shutdown();

    return 0;
}
