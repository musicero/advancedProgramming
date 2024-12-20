use crate::game::{Cell, Game};

#[derive(Debug)]
pub enum CmdErr {
    InvalidCommand,
    InvalidArgument(&'static str),
    IllegalCommand(&'static str),
}

pub struct Reply {
    pub this: Option<String>,
    pub other: Option<String>,
}

fn format_error(err: CmdErr) -> Reply {
    let msg = match err {
        CmdErr::InvalidArgument(msg) => format!("Invalid argument: {}", msg),
        CmdErr::IllegalCommand(msg) => format!("Illegal command: {}", msg),
        CmdErr::InvalidCommand => "Invalid command".to_string(),
    };
    Reply {
        this: Some(format!("ERR: {}", msg)),
        other: None,
    }
}

pub fn handle_command(game: &mut Game, input: String) -> Reply {
    let mut words = input.split_whitespace();
    if let Some(command) = words.next() {
        return match command {
            "newgame" => match new_game(game, words.collect()) {
                Ok(reply) => reply,
                Err(err) => format_error(err),
            },
            "move" => match do_move(game, words.collect()) {
                Ok(reply) => reply,
                Err(err) => format_error(err),
            },
            "groups" => print_groups(game),
            "board" => print_game(game),
            "quit" | "exit" => Reply {
                this: None,
                other: Some("Opponent disconnected".to_string()),
            },
            "help" => print_help(),
            _ => format_error(CmdErr::InvalidCommand),
        };
    } else {
        Reply {
            this: Some("Empty command".to_string()),
            other: None,
        }
    }
}

fn new_game(game: &mut Game, args: Vec<&str>) -> Result<Reply, CmdErr> {
    let mut args = args.into_iter();
    let size = args
        .next()
        .ok_or(CmdErr::InvalidArgument("no size given"))?
        .parse::<usize>()
        .map_err(|_| CmdErr::InvalidArgument("size must be an int"))?;

    *game = Game::new((size, size));
    let reply = Reply {
        this: Some(format!("New game with size {}", size)),
        other: Some(format!("New game with size {}", size)),
    };
    Ok(reply)
}

fn print_help() -> Reply {
    Reply {
        this: Some(
            "Commands:\n\
            newgame <size> - start new game with size\n\
            move <x> <y> - place stone at x, y\n\
            groups - print number of groups\n\
            board - print current board\n\
            quit - exit the game\n\
            help - print this message"
                .to_string(),
        ),
        other: None,
    }
}

fn do_move(game: &mut Game, args: Vec<&str>) -> Result<Reply, CmdErr> {
    let mut args = args.into_iter();

    let y = args
        .next()
        .ok_or(CmdErr::InvalidArgument("No Y given"))?
        .parse::<usize>()
        .map_err(|_| CmdErr::InvalidArgument("Y must be an int"))?
        .checked_sub(1)
        .ok_or(CmdErr::InvalidArgument("Coords start at 1"))?;

    let x = args
        .next()
        .ok_or(CmdErr::InvalidArgument("No X given"))?
        .parse::<usize>()
        .map_err(|_| CmdErr::InvalidArgument("X must be an int"))?
        .checked_sub(1)
        .ok_or(CmdErr::InvalidArgument("Coords start at 1"))?;

    if x >= game.width || y >= game.height {
        return Err(CmdErr::IllegalCommand("Coordinates must be inside board"));
    }
    game.place_stone((x, y))
        .map_err(|_| CmdErr::IllegalCommand("out of bound"))?;
    game.remove_dead();

    Ok(Reply {
        this: Some(format!("You placed stone at ({}, {})", x, y)),
        other: Some(format!("Opponent placed stone at ({}, {})", x, y)),
    })
}

fn print_game(game: &mut Game) -> Reply {
    let board = game.get_board();
    Reply {
        this: Some(board),
        other: None,
    }
}

fn print_groups(game: &mut Game) -> Reply {
    let groups = game.get_groups();
    let white = groups
        .iter()
        .filter(|group| group.color == Cell::White)
        .count();
    // could also have done black = len() - white, but this is more readable
    let black = groups
        .iter()
        .filter(|group| group.color == Cell::Black)
        .count();
    let message = format!("White groups: {}\nBlack groups: {}", white, black);
    let reply = Reply {
        this: Some(message.clone()),
        other: Some(message),
    };
    reply
}
