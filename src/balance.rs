use std::collections::HashMap;
use super::expenses::Expenses;

pub fn get_spend_diff(all_expenses: &Expenses) -> HashMap<String,f64> {
	let total_spend = get_total_spend(all_expenses);
	let per_person_spend = total_spend / (all_expenses.people.len() as f64);
    println!("Total Spend is {}, Divided by {}, cost each is {}",total_spend,all_expenses.people.len(),per_person_spend);

    for (person,personal_expenses) in all_expenses.people.iter() {
        let diff = personal_expenses.personal_total_spend - per_person_spend;
        if diff.is_sign_positive() {
            println!("{person} is owed {}",diff);
        }
        else {
            println!("{person} owes {}",-diff);
        }
    }
}

pub fn calculate_settle_amounts(spend_diff: HashMap<String,f64>,all_expenses: Expenses) {
	println!("\nTo Settle:");
    for receive_person in all_expenses.clone().people.keys() {
        if all_expenses.people[receive_person].personal_total_spend <= per_person_spend {
            continue;
        }
        for give_person in all_expenses.clone().people.keys()  {
            //take from those who owe in order
            if all_expenses.people[give_person].personal_total_spend >= per_person_spend {
                continue;
            }

            let to_receive = all_expenses.people[receive_person].personal_total_spend - per_person_spend;
            let to_give = per_person_spend - all_expenses.people[give_person].personal_total_spend;
            let give_amount = (to_give).min(to_receive);
            all_expenses.people.get_mut(receive_person).unwrap().personal_total_spend -= give_amount;
            all_expenses.people.get_mut(give_person).unwrap().personal_total_spend += give_amount;

            println!("{give_person} Sends {} euros to {receive_person}",give_amount);
        }
    }
}

fn get_total_spend(all_expenses: &Expenses) -> f64 {
	let mut total_spend = 0.0;
	for (_person,personal_expenses) in all_expenses.people.iter() {
			total_spend+= personal_expenses.personal_total_spend;
	}
	total_spend
}