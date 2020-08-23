use console::Term;
use rbp_core::budget_group::BudgetGroup;
use std::ptr::null;
use std::mem::MaybeUninit;

struct Tui {
    ui: Term,
    budget: MaybeUninit<BudgetGroup>
}


impl Tui {
    /// Start the text-based user interface in the bound console.
    ///
    /// Note that this method call does not return until the user exits the program.
    fn start() {
        let tui = Tui { ui: Term::stdout(), budget: MaybeUninit::uninit()};
        tui.run();
    }

    fn run(&self) {
        self.ui.write_line("Welcome to the Rust Budget Planner");
    }
}