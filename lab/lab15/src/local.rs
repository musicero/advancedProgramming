use std::io::{self, Write};

use crate::game::Game;
use crate::game_cmd::*;

pub fn local() {
    let mut game = Game::new((19, 19));
    loop {
        handle_input(&mut game);
    }
}

fn handle_input(game: &mut Game) {
    // take input
    print!("Enter command: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");
    let input = input.trim();

    if input == "exit" {
        std::process::exit(0);
    }

    let reply = handle_command(game, input.to_string());
    match reply.this {
        Some(msg) => println!("{}", msg),
        None => (),
    }
}
