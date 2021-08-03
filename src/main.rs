pub mod storage;

fn main() {
    let storage_path = "src/data/storage_test.toml";
    println!("Hello, world!");

    let environment = storage::parse_toml::<storage::Environment>("src/data/env.toml");
    let mut data = storage::parse_toml::<storage::Storage>(&storage_path);
    println!("Owner name: {}\n", environment.owner);
    println!("List of servers:");
    for (key, _) in &data.servers {
        println!("{}", key);
    }
    data.servers.insert(
        "23156133".to_string(),
        storage::Server {
            bot_channels: None,
            pronoun_roles: None,
        }
    );
    println!("\n\nNew list of servers:");
    for (key, _) in &data.servers {
        println!("{}", key);
    }
    storage::write_toml::<storage::Storage>(&storage_path, &data);
}