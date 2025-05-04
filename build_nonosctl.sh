#!/bin/bash

set -e

echo "[NØNOS CLI] Building full command structure..."

APP_DIR="modules/nonosctl"
SRC_DIR="$APP_DIR/src"
CLI_DIR="$SRC_DIR/cli"
MOD_DIR="$SRC_DIR/modules"

mkdir -p "$CLI_DIR" "$MOD_DIR"

# 1. Create Cargo.toml
cat > "$APP_DIR/Cargo.toml" <<EOF
[package]
name = "nonosctl"
version = "1.0.0"
edition = "2021"

[dependencies]
EOF

# 2. Create src/main.rs
cat > "$SRC_DIR/main.rs" <<EOF
mod cli;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: nonosctl <module> <command>");
        return;
    }

    let module = &args[1];
    let command = &args[2];
    cli::dispatch(module, command);
}
EOF

# 3. Create src/cli/mod.rs
cat > "$CLI_DIR/mod.rs" <<EOF
use crate::modules::*;

pub fn dispatch(module: &str, command: &str) {
    match module {
        "fs" => fs::handle(command),
        "net" => net::handle(command),
        "sys" => sys::handle(command),
        "user" => user::handle(command),
        "zk" => zk::handle(command),
        "deploy" => deploy::handle(command),
        "relay" => relay::handle(command),
        _ => eprintln!("Unknown module: {}", module),
    }
}
EOF

# 4. Stub out all modules with handle() method
for mod in fs net sys user zk deploy relay; do
  cat > "$MOD_DIR/$mod.rs" <<EOF
pub fn handle(cmd: &str) {
    println!("[$mod] Handling command: {}", cmd);
}
EOF
done

# 5. Create modules/mod.rs to link them
cat > "$MOD_DIR/mod.rs" <<EOF
pub mod fs;
pub mod net;
pub mod sys;
pub mod user;
pub mod zk;
pub mod deploy;
pub mod relay;
EOF

echo "[✓] CLI command structure created."
