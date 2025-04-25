mod cli;
mod modules;

use cli::run;
use modules::{fs, net, user, deploy, sys, zk, relay};

fn main() {
    println!("Welcome to NØNOS CLI");

    fs::tree();
    net::status();
    user::profile();
    deploy::push("file.pkg");
    sys::info();
    zk::verify();
    relay::connect();
}
