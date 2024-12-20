use std::io::Write;
mod constants;

mod game;
mod game_cmd;

mod client;
mod local;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Choose mode: [server|client|local]");
        print!("> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.to_lowercase().trim() {
            "server" => server::server(),
            "client" => client::client(),
            "local" => local::local(),
            _ => (),
        }
    }
}
