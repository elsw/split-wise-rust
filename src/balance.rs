use std::collections::HashMap;
use super::expenses::Expenses;

pub struct Balance {
    //Array of poeple and required payments
    payments: HashMap<String,Payments>,
}

pub struct Payments {
    //Array of poeple to pay
    poeple: HashMap<String,f64>,
}

impl Balance {
	pub fn new() -> Balance {
		Balance { 
			payments: HashMap::new()
		}
	}
}
impl Payments {
	pub fn new() -> Payments {
		Payments {
			poeple: HashMap::new()
		}
	}
}

pub fn total_spending(expenses: &Expenses) -> Balance{
	let mut balance = Balance::new();
    for expense in expenses.expenses.iter() {
        let num_buyers = expense.brought_by.len() as f64;
        let num_for = expense.expense_for.len() as f64;

		//Calculate Amount of money
		let mut amount = expense.amount;
		if expense.currency != expenses.main_currency {
			let key: String = expense.currency.to_string();
			let conversion: f64 = expenses.currencies.get(&key).unwrap().clone();
			amount *= conversion;
		}
		amount /= num_for;
		amount /= num_buyers;

		//For payments due, add the person if not yet added
		for expense_name in expense.expense_for.iter() {
			if !balance.payments.contains_key(expense_name) {
				balance.payments.insert(expense_name.clone(), Payments::new());
			}

			//Add Payment to each buyer
			for buyer_name in expense.brought_by.iter() {
				//Skip self expenses
				if expense_name.eq(buyer_name) {
					continue;
				}

				//Append a payment to the list of payments for the person
				if balance.payments.get(expense_name).unwrap().poeple.contains_key(buyer_name) {
					balance.payments.get_mut(expense_name).unwrap().poeple.insert(buyer_name.clone(), amount);
				} else {
					let mut current_amount: &mut f64 = balance.payments.get_mut(expense_name).unwrap().poeple.get_mut(buyer_name).unwrap();
					*current_amount += amount;
				}
			}

		}
    }
	balance
}

/*pub fn group_expenses(balance: &Balance,groups) {

}*/