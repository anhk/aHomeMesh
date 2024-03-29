mod args;
mod client;
mod grpc;
mod server;
mod tun;

use env_logger::{Builder, Target};
use log::info;
use std::io::Read;
use tun::Tun;

#[tokio::main]
async fn main() {
    // env_logger init
    let mut log_builder = Builder::from_default_env();
    log_builder
        .target(Target::Stdout)
        .filter(None, log::LevelFilter::Debug)
        .init();
    // env_logger::init();
    match args::parse() {
        args::Command::Server(option) => tokio::spawn(async move { server::listen(&option) }),
        args::Command::Client(option) => tokio::spawn(async move { client::connect(&option) }),
    };

    let mut data = [0u8; 2048];
    let mut tun = tun::alloc_tun().unwrap();
    tun.add_route("34.34.34.0/24").unwrap();

    while match tun.read(&mut data) {
        Ok(size) => {
            info!("read {} from tun", size);
            process(&data[..size]);
            true
        }
        Err(_e) => false,
    } {}
    println!("Hello, world!");
}

fn process(_data: &[u8]) {}
