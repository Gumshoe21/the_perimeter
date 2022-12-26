pub mod game;

fn main() {
    println!("Welcome to The Perimeter.");
    println!();

    let mut command = game::Command::new();

    let mut output: String;

    while command.verb != "quit" {
        command = game::get_input();
        output = game::update_state(&command);
        game::update_screen(output);
    }

    println!("Goodbye.")
}
