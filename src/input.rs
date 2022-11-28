use std::io;
use std::io::Write;

pub fn string(title: &str, default_val: &str) -> String {
    let mut input = String::from(default_val);

    print!("🍀 Enter {} ({}): ", title, default_val);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect(&("Error getting".to_owned() + title));

    input.trim().to_string()
}

pub fn optional_string(title: &str, example: &str) -> Option<String> {
    let mut input = String::new();

    print!("🍀 Enter {} ({}): ", title, example);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect(&("Error getting".to_owned() + title));

    if input.trim().is_empty() {
        None
    } else {
        Some(input.trim().to_string())
    }
}
