extern crate rbp_core;

use rbp_core::budget_group::BudgetGroup;

#[test]
fn create_budget_group() {
    BudgetGroup::new("Test group");
}