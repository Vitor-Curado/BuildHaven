fn main() {
    if let Err(e) = buildhaven::assets::build_assets() {
        eprintln!("Asset build failed: {}", e);
        std::process::exit(1);
    }

    println!("Assets built successfully.");
}
