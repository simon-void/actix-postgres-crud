# actix-postgres-crud

## initial setup
- [install Rust](https://www.rust-lang.org/learn/get-started), on Mac/Linux via
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- install docker and docker-compose
```shell
sudo apt install docker
sudo apt install docker-compose
```

## steps to compile/run program natively (without docker)
- compile and run program
```shell
cargo build --release
./target/release/actix-postgres-crud
```

## steps to deploy in docker
- build the program, than the docker image, than start it 
```shell
cargo build --release
sudo docker-compose build
sudo docker-compose up
```