pub mod expenses;
pub mod balance;

fn main() {
    let filename = String::from("input.toml");
    let mut all_expenses = expenses::Expenses::new();
	all_expenses.read_from_file(filename);
	all_expenses.total_spending();
    //all_expenses.print_spend_breakdown();

    //spend_diffs = get_spend_diff(all_expenses);
	//calculate_settle_amounts(spend_diffs);
}