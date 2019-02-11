#ifndef __MESSEND_H__
#define __MESSEND_H__

#include <stdint.h>
#include <stdbool.h>
#include <SDL2/SDL_net.h>

#ifdef __cplusplus
extern "C"
{
#endif

typedef struct _Acceptor* MessendAcceptor;
typedef struct _Peer* MessendPeer;

typedef struct {
    uint8_t* data;
    uint64_t size;
} MessendMessage;

void messend_startup();
void messend_shutdown();

MessendAcceptor messend_acceptor_create(const char* host, uint16_t port);
MessendPeer messend_acceptor_accept(MessendAcceptor acceptor);
MessendPeer messend_acceptor_accept_wait(MessendAcceptor acceptor);
void messend_acceptor_free(MessendAcceptor acceptor);

MessendPeer messend_initiate(char* addr, int port);

bool messend_peer_is_connected(MessendPeer peer);
void messend_peer_send_message(MessendPeer peer, MessendMessage message);
bool messend_peer_has_message(MessendPeer peer);
MessendMessage* messend_peer_receive_message(MessendPeer peer);
MessendMessage* messend_peer_receive_message_wait(MessendPeer peer);
void messend_peer_free(MessendPeer peer);

void messend_message_free(MessendMessage* message);

#ifdef __cplusplus
}
#endif

#endif //__MESSEND_H__
