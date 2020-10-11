cargo build --release 
cp ./target/release/main ./main
scp ./main root@io.div.is:~/main
#export datbaseurl
#start redis-server
./main
