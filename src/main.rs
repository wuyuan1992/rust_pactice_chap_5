use std::{fmt, io};

#[derive(Debug)]
struct Player {
    name: String,
    x: i32,
    y: i32,
}

impl Player {
    // Define your `move_player` method here
    fn move_player(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

impl fmt::Display for Player {
    // Implement the `Display` trait for the `Player` struct here
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is at ({}, {})", self.name, self.x, self.y)
    }
}

fn main() {
    let mut player = Player {
        name: String::from("Wuyuan"),
        x: 0,
        y: 0,
    };

    println!("hey {}, Let's move", player.name);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim();

        match command {
            "b" => player.move_player(0, 1),
            "t" => player.move_player(0, -1),
            "r" => player.move_player(1, 0),
            "l" => player.move_player(-1, 0),
            "q" => break,
            _ => {
                println!("Unknown command: {}", command);
                break;
            }
        }

        println!("{}", player);
    }
}
