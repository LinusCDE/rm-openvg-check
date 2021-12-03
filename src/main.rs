use egli::Display;

fn main() -> egli::error::Result<()> {
    println!("Extensions:");
    for ext in egli::query_extensions() {
        if ext.contains(" ") {
            for ext in ext.split(" ") {
                println!(" - {}", ext);
            }
        } else {
            println!(" - {}", ext);
        }
    }

    println!("Opending display...");
    let display = Display::from_default_display()?;
    println!("Opened display");
    display.initialize()?;
    println!("Initialized display");
    println!("Client APIs:");
    for api in display.query_client_apis() {
        if api.contains(" ") {
            for api in api.split(" ") {
                println!(" - {}", api);
            }
        } else {
            println!(" - {}", api);
        }
    }

    println!("Vendor: {}", display.query_vendor()?);
    println!("Version: {}", display.query_version()?);

    display.create_context
    Ok(())
}
