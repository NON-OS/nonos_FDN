use std::{thread, time::Duration};
use colored::*;

fn print_delay(msg: &str, delay: u64) {
    println!("{}", msg);
    thread::sleep(Duration::from_millis(delay));
}

fn main() {
    println!("{}", "NØNOS Terminal v0.1.2-alpha".bright_green().bold());
    println!();

    print_delay(" :: Initializing hardened memory allocator", 300);
    print_delay(" :: MAC address randomization... done", 300);
    print_delay(" :: Zero-State Boot Mode activated", 300);
    print_delay(" :: Securing kernel modules with AppArmor", 300);
    print_delay(" :: Initializing encrypted relay mesh...", 300);
    print_delay(" :: Relay mesh active", 300);
    println!();

    print_delay(" :: PGP keyring loaded from ephemeral memory", 300);
    println!();
    println!("{}", "NØNOS ready  >  Type 'help' for available commands".bright_green().bold());
    println!();
    print!("{}", "non@zero-state:$ ".green());
}
