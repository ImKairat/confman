mod cli;
use cli::{get_args, Commands};

fn main() {
    match get_args().unwrap() {
        Commands::Config {
            file_name: file,
            file_path: path
        } => println!("Config: {path}{file}"),
        Commands::Start {verbose} => println!("Start: {verbose}"),
        Commands::Init => println!("Init"),
        Commands::Sync {url}=> {
            match url {
                Some(u) => println!("Sync with {u}"),
                None => {}
            }
        },
    }
}
