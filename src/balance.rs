use std::collections::HashMap;
use crate::expenses;

use super::expenses::Expenses;
use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
pub struct Balance {
    //Array of poeple and required payments
    pub payments: HashMap<String,Payments>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Payments {
    //Array of poeple to pay
    pub people_to_pay: HashMap<String,f64>,
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
			people_to_pay: HashMap::new()
		}
	}
}
impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (person,payments) in self.payments.iter() {
            write!(f, "{person} Pays: ")?;
            for (person_to_pay, amount) in payments.people_to_pay.iter() {
                write!(f, "{person_to_pay} £{amount:.1}, ")?;
            }
            writeln!(f,"")?;
        }
        Ok(())
    }
}


pub fn total_spending(expenses: &Expenses) -> Balance{
	let mut balance = Balance::new();

    for expense in expenses.expenses.iter() {
        let num_buyers = expense.brought_by.len();
        let mut poeple_for = expense.expense_for.clone();

        if poeple_for.len() == 0 {
            //This means the spend is for everyone
            poeple_for = expenses.people.clone();
        }
        let num_for = poeple_for.len();

        let mut total_weights= 0.0;
        for person in poeple_for.iter() {
            total_weights +=  expenses.weights.get(person).unwrap();
        }

		//Calculate Total Amount of money
		let mut amount = expense.amount;
		if expense.currency != expenses.main_currency {
			let key: String = expense.currency.to_string();
			let conversion: f64 = expenses.currencies.get(&key).unwrap().clone();
			amount *= conversion;
		}

		//For payments due, add the person if not yet added
		for expense_name in poeple_for.iter() {
            let payments = balance.payments.entry(expense_name.clone()).or_insert(Payments::new());

			//Add Payment to each buyer
			for buyer_name in expense.brought_by.iter() {
				if expense_name.eq(buyer_name) {
                    //Skip self expenses
					continue;
				}
                let amount_to_pay = (amount / total_weights) * expenses.weights.get(expense_name).unwrap();

				//Append a payment to the list of payments for the person
                *payments.people_to_pay.entry(buyer_name.clone()).or_insert(0.0) += amount_to_pay;
			}

		}
    }
	balance
}

/*pub fn group_expenses(balance: &Balance,groups) {

}*/