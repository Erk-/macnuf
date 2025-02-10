fn main() {
    match macnuf::lookup("00:18:23:ac:09:02".parse().unwrap()) {
        Some(manuf) => {
            println!("Manufacturer: {}", manuf)
        }
        None => {
            println!("No manufacturer found")
        }
    }
}
