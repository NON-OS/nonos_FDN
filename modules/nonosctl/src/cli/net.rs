pub async fn run(command: &str) {
    match command {
        "boot" => {
            println!("Bringing up mesh interface...");
            // Mesh init logic here
        }
        "status" => {
            println!("Querying mesh network...");
            // Status logic
        }
        _ => {
            eprintln!("Unknown net command: {}", command);
        }
    }
}
