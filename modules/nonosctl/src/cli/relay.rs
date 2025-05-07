pub async fn run(command: &str) {
    match command {
        "start" => {
            println!("Starting encrypted relay node...");
            // Relay start logic
        }
        "stop" => {
            println!("Stopping relay...");
        }
        _ => {
            eprintln!("Unknown relay command: {}", command);
        }
    }
}
