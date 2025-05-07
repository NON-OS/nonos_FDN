pub async fn run(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        "deploy" => {
            println!("Deploying payload...");
            // logic here
        },
        _ => println!("Unknown deploy command"),
    }
    Ok(())
}

