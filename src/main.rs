use crate::balance::Payments;

pub mod expenses;
pub mod balance;

fn main() {
    let filename = String::from("input.toml");
    let mut all_expenses = expenses::Expenses::new();
	all_expenses.read_from_file(filename);

    for expense in all_expenses.expenses.iter() {
        println!("{expense}")
    }
    println!("");

    for (person,weight) in all_expenses.weights.iter() {
        println!("{person} is {weight} poeple");
    }
    println!("");

    let total_spend = all_expenses.total_spend();
    for (person, spend) in total_spend.iter() {
        println!("{person} spent a total of {}{spend}",all_expenses.main_currency);
    }
    println!("");

    let mut balances = balance::total_spending(&all_expenses);
    println!("{balances}");

    println!("Simplified:");
    for (payer,payments) in balances.payments.clone().iter() {
        for (pay_to, amount) in payments.people_to_pay.clone().iter() {
            if balances.payments.contains_key(pay_to) {
                let [a,b] = balances.payments.get_disjoint_mut([pay_to,payer]);
                let balance_pay_to = a.expect("no balance_pay_to");
                let balance_pay_from = b.expect("no balance_pay_from");
                if balance_pay_to.people_to_pay.contains_key(payer) {
                    //There is a 2 way payment, simplify
                    let mut amount = balance_pay_from.people_to_pay.get_mut(pay_to).unwrap();
                    let mut other_amount = balance_pay_to.people_to_pay.get_mut(payer).unwrap();
                    //Remove whichever entry is lower, and minus the difference
                    if amount > other_amount {
                       //balance_pay_to.people_to_pay.remove(payer);
                        *amount -= *other_amount;
                        *other_amount = 0.0;
                    }
                    else {
                        //balances.payments.get(payer).unwrap().people_to_pay.remove(pay_to);
                        *other_amount -= *amount;
                        *amount = 0.0;
                    }
                }   
            }
        }
    }

     for (payer,payments) in balances.payments.clone().iter() {
        for (pay_to, amount) in payments.people_to_pay.clone().iter() {
            if *amount <= 0.0 {
                balances.payments.get_mut(payer).unwrap().people_to_pay.remove(pay_to);
            }
        }
        if balances.payments.get_mut(payer).unwrap().people_to_pay.len() == 0 {
            balances.payments.remove(payer);
        }
    }    

    println!("{balances}");

}