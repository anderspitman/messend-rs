clang -std=c99 server.c -o server -I ../../messend -lmessend -L ../target/debug/ -ldl -lpthread
clang -std=c99 client.c -o client -I ../../messend -lmessend -L ../target/debug/ -ldl -lpthread
