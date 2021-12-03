use khronos_egl as egl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance = egl::Instance::new(egl::Static);

    println!("Version: {}", instance.version());
    /*
    println!("Extensions:");
    for ext in egl::query_extensions() {
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

    display.create_context*/
    Ok(())
}
