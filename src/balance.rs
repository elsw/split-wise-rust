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

pub fn total_spending(expenses: &Expenses) -> Balance{
    for expense in self.expenses.iter_mut() {
        let num_buyers = expense.brought_by.len() as f64;
        //let num_for = expense.expense_for.len() as f64;
        let mut amount = expense.amount;
        if expense.currency != self.main_currency {
            let key: String = expense.currency.to_string();
            let conversion: f64 = self.currencies.get(&key).unwrap().clone();
            amount *= conversion;
        }
        amount /= num_buyers;

        for name in expense.brought_by.iter_mut() {
            if !self.people.contains_key(name) {
                self.people.insert(name.clone(), 0.0);
            }
            let named_amount = self.people.get_mut(name).unwrap();
            *named_amount += amount;
        }
    }
}