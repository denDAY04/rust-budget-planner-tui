mod tui;

use console::Term;

fn main() {
    let terminal = Term::stdout();
    terminal.write_line("Welcome to the Rust Budget Planner");
}
