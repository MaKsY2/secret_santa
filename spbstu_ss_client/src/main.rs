use std::collections::HashMap;
use std::process::exit;
use std::string::ToString;

use reqwest::blocking::Client;
use reqwest::StatusCode;

struct State {
    pub token: String,
    pub user_id: i32,
    pub name: String
}

static mut state: State = State {
    token: String::new(),
    user_id: -1,
    name: String::new()
};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String
}

fn exit_handler(_args: Vec<&str>) {
    exit(0);
}

unsafe fn login_handler(args: Vec<&str>) {
    if state.user_id != -1 {
        println!("You are already logged in.")
    }
    let username = args.first().unwrap();
    let password = args[1];
    let client = Client::builder().build().unwrap();
    let mut map = HashMap::new();
    map.insert("name", username);
    map.insert("password", &password);

    let response = client.post("http://localhost:8000/login")
        .json(&map).send().unwrap();
    state.token = response.json::<Token>().unwrap().token;
    println!("Login successful!");
}

unsafe fn logout_handler(args: Vec<&str>) {
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

unsafe fn groups_handler(args: Vec<&str>) {
    let sub_option = args.first().unwrap_or(&"help");
    match sub_option {
        &"help" => {
            println!("groups help - list of this commands");
            println!("groups list - list of all groups");
            println!("groups create <name> - create group");
            println!("groups join <id> - join group");
            println!("groups leave <id> - leave group");
            println!("groups makesantas <id> - make santas in group and close it");
            println!("groups receiver <id> - check your receiver in group");
        },
        &"list" => {
            let client = Client::builder().build().unwrap();
            let response = client.get("http://localhost:8000/groups").send().unwrap();
            let groups = response.json::<Vec<Group>>().unwrap();
            for group in groups {
                println!("{} {}", group.group_id, group.name);
            }
        },
        &"create" => {
            if args[1].len() == 0 {
                return println!("give a name");
            }
            let data = CreateGroup {
                name: args[1].to_string()
            };
            let client = Client::builder().build().unwrap();
            let response = client.post("http://localhost:8000/groups")
                .header("token", &state.token)
                .json(&data)
                .send().unwrap();
            let new_group = match response.json::<Group>() {
                Ok(group) => group,
                Err(e) => {return println!("Please log in")}
            };
            println!("mew group: {} {}", new_group.group_id, new_group.name);
        },
        &"join" => {
            if args[1].len() == 0 {
                return println!("give an id");
            }
            let data = CreateMembership {
                user_id: state.user_id,
                group_id: args[1].to_string().parse::<i32>().unwrap()
            };
            let client = Client::builder().build().unwrap();
            let response = client.post("http://localhost:8000/memberships")
                .header("token", &state.token)
                .json(&data)
                .send().unwrap();
            match response.json::<Membership>() {
                Ok(_t) => println!("Ok"),
                Err(_e) => println!("You are unable to join this group")
            }
        },
        &"leave" => {
            if args[1].len() == 0 {
                return println!("give an id");
            }
            let client = Client::builder().build().unwrap();
            let response = client.delete(
                format!("http://localhost:8000/memberships?user_id={}&group_id={}", state.user_id.to_string(), args[1])
            )
                .header("token", &state.token)
                .send()
                .unwrap();
            match response.status() {
                StatusCode::OK => println!("Ok"),
                _ => println!("Error")
            }
        },
        &"makesantas" => {
            if args[1].len() == 0 {
                return println!("give an id");
            }
            let client = Client::builder().build().unwrap();
            let response = client.post(
                format!("http://localhost:8000/santas?group_id={}", args[1])
            )
                .header("token", &state.token)
                .send()
                .unwrap();
            match response.status() {
                StatusCode::OK => println!("Ok"),
                _ => println!("Error")
            }
        }
        _ => {
            println!("Unknown command!")
        }
    }
}

unsafe fn user_input(handlers: &HashMap<String, unsafe fn(Vec<&str>)>, input: String) {
    let mut split = input.split(" ");
    let command = match split.next() {Some(a) => a, None => return};
    let args = split.collect::<Vec<&str>>();
    match handlers.get(command) {
        Some(func) => func(args),
        None => println!("Unknown command")
    };
}

#[derive(Serialize)]
struct CreateUser {
    pub name: String,
    pub password: String
}

unsafe fn signup_handler(args: Vec<&str>) {
    let data = CreateUser {
        name: args[0].to_string(),
        password: args[1].to_string()
    };
    let client = Client::builder().build().unwrap();
    let response = client.post("http://localhost:8000/users")
        .send()
        .unwrap();
    match response.status() {
        StatusCode::OK => println!("successfully signed up"),
        _ => print!("Something went wrong")
    }
}

fn main() {
    let mut buffer = String::new();
    let mut handlers: HashMap<String, unsafe fn(Vec<&str>)> = HashMap::new();
    handlers.insert("exit".to_string(), exit_handler);
    handlers.insert("login".to_string(), login_handler);
    handlers.insert("logout".to_string(), logout_handler);
    handlers.insert("signup".to_string(), signup_handler);
    handlers.insert("groups".to_string(), groups_handler);
    handlers.insert("signup".to_string(), signup_handler);

    while std::io::stdin().read_line(&mut buffer).is_ok() {
        unsafe {
            user_input(
                &handlers,
                buffer.strip_suffix("\n").unwrap().to_string()
            );
        }
        buffer = "".to_string();
    }
}
