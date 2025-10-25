use std::env;

#[derive(Debug)]
enum Command {
    UNKNOWN,
    HELP,
    INIT,
    RUN,
}

impl From<&str> for Command {
    fn from(string: &str) -> Command {
        match string.to_uppercase().as_str() {
            "HELP" => Command::HELP,
            "INIT" => Command::INIT,
            "RUN" => Command::RUN,
            _ => Command::UNKNOWN
        }
    }
}

fn main() {
    let _args = env::args().collect::<Vec<String>>();
    println!("Args: {:?}", _args);

}

