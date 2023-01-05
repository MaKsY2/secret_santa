use std::collections::HashMap;
use std::process::exit;

fn exit_handler(args: Vec<&str>) {
    exit(0);
}

fn user_input(handlers: &HashMap<String, fn(Vec<&str>)>, input: String) {
    let mut split = input.split(" ");
    let command = match split.next() {Some(a) => a, None => return};
    let args = split.collect::<Vec<&str>>();
    match handlers.get(command) {
        Some(func) => func(args),
        None => println!("Unknown command")
    };
}

fn main() {
    let mut buffer = String::new();
    let mut handlers: HashMap<String, fn(Vec<&str>)> = HashMap::new();
    handlers.insert("exit".to_string(), exit_handler);

    while std::io::stdin().read_line(&mut buffer).is_ok() {
        user_input(
            &handlers,
            buffer.strip_suffix("\n").unwrap().to_string()
        );
        buffer = "".to_string();
    }
}
