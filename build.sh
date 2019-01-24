clang -std=c99 ../messend/examples/server.c -o server -I ../messend -lmessend -L ./target/debug/ -ldl -lpthread
clang -std=c99 ../messend/examples/client.c -o client -I ../messend -lmessend -L ./target/debug/ -ldl -lpthread
