use std::collections::HashMap;
use std::process::exit;
use std::string::ToString;

use reqwest::blocking::Client;

struct State {
    pub token: String,
    pub user_id: i32,
    pub name: String
}

static mut state: State = State {
    token: "".to_string(),
    user_id: -1,
    name: "-".to_string()
};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String
}

fn exit_handler(_args: Vec<str>) {
    exit(0);
}

unsafe fn login_handler(args: Vec<str>) {
    if state.user_id != -1 {
        println!("You are already logged in.")
    }
    let username = args.first().unwrap();
    let client = Client::builder().build().unwrap();
    let mut map = HashMap::new();
    map.insert("name", username);

    let response = client.post("http://localhost:8000/login")
        .json(&map).send().unwrap();
    state.token = response.json::<Token>().unwrap().token;
    println!("Login successful!");
}

unsafe fn logout_handler(args: Vec<str>) {
    state.user_id = -1;
    state.token = "".to_string();
    state.name = "-".to_string();
    println!("Logout succefful.")
}

#[derive(Deserialize)]
struct Group {
    group_id: i32,
    name: String
}

#[derive(Serialize)]
struct CreateGroup {
    pub name: String
}

#[derive(Deserialize)]
struct Membership {
    pub user_id: i32,
    pub group_id: i32,
    pub role: String
}

#[derive(Serialize)]
struct CreateMembership {
    pub user_id: i32,
    pub group_id: i32
}

unsafe fn groups_handler(args: Vec<str>) {
    let sub_option = args.first().unwrap_or(&"help");
    match sub_option {
        "help" => {
            println!("groups help - list of this commands");
            println!("groups list - list of all groups");
            println!("groups create <name> - create group");
            println!("groups join <id> - join group");
            println!("groups leave <id> - leave group");
            println!("groups receiver <id> - check your receiver in group");
        },
        "list" => {
            let client = Client::builder().build().unwrap();
            let response = client.get("http://localhost:8000/groups").send().unwrap();
            let groups = response.json::<Vec<Group>>().unwrap();
            for group in groups {
                println!("{} {}", group.group_id, group.name);
            }
        },
        "create" => {
            if args[1].is_none() {
                return println!("give a name");
            }
            let data = CreateGroup {
                name: args[1].unwrap()
            };
            let response = client.post("http://localhost:8000/groups")
                .header("token", &state.token)
                .json(&data)
                .send().unwrap();
            let new_group = match response.json::<Group>().unwrap() {
                Ok(group) => group,
                Err(e) => {return println!("Please log in")}
            };
            println!("mew group: {} {}", new_group.group_id, new_group.name);
        },
        "join" => {
            if args[1].is_none() {
                return println!("give an id");
            }
            let data = CreateMembership {
                user_id: state.user_id,
                group_id: args[1].unwrap()
            };
            let response = client.post("http://localhost:8000/memberships")
                .header("token", &state.token)
                .json(&data)
                .send().unwrap();
            match response.json::<Membership>().unwrap() {
                Ok(_t) => println!("Ok"),
                Err(_e) => println!("You are unable to join this group")
            }
        },
        "leave" => {
            if args[1].is_none() {
                return println!("give an id");
            }
            let data = CreateMembership {
                user_id: state.user_id,
                group_id: args[1].unwrap()
            };
        }
        _ => {
            println!("Unknown command!")
        }
    }
}

fn user_input(handlers: &HashMap<String, fn(Vec<str>)>, input: String) {
    let mut split = input.split(" ");
    let command = match split.next() {Some(a) => a, None => return};
    let args = split.collect::<Vec<str>>();
    match handlers.get(command) {
        Some(func) => func(args),
        None => println!("Unknown command")
    };
}

fn main() {
    let mut buffer = String::new();
    let mut handlers: HashMap<String, fn(Vec<str>)> = HashMap::new();
    handlers.insert("exit".to_string(), exit_handler);
    handlers.insert("login".to_string(), login_handler);
    handlers.insert("logout".to_string(), logout_handler);

    while std::io::stdin().read_line(&mut buffer).is_ok() {
        user_input(
            &handlers,
            buffer.strip_suffix("\n").unwrap().to_string()
        );
        buffer = "".to_string();
    }
}
