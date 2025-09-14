use std::fs;
use toml::Table;
use toml::Value;
use std::collections::HashMap;

#[derive(Clone)]
struct Expenses {
    people: HashMap<String,PersonalExpenses>,
}

#[derive(Clone)]
struct PersonalExpenses {
    expenses: HashMap<String,f64>,
    personal_total_spend: f64,
}

fn read_from_file(filename: &String) -> Expenses {
    let input = match fs::read_to_string(filename)
    {
        Ok(file) => file,
        Err(error) => panic!("Error opening file {error}"),
    };
    
    let data: Table = match toml::from_str(input.as_str())
    {
        Ok(value) => value,
        Err(error) => panic!("Could not parse toml: {error}"),
    };
    
    let mut all_expenses = Expenses {people: HashMap::new()};

    for (key, value) in data.iter() {
        match value {
            Value::Table(table) => {
                let mut personal_expenses = PersonalExpenses { expenses: HashMap::new(),personal_total_spend: 0.0 };

                for (key, value) in table{
                    if let Value::Float(val) = value {
                        personal_expenses.expenses.insert(key.clone(), val.clone());
                    }
                    else if let Value::Integer(val) = value {
                        personal_expenses.expenses.insert(key.clone(), val.clone() as f64);
                    }
                    else {
                        println!("Got invalid value on second level {}",value);
                    }
                }
                all_expenses.people.insert(key.clone(),personal_expenses);
            }
            _ => println!("First level should only be name labels, got: {}", value),
        }
    }
    all_expenses
}

fn main() {
    let filename = String::from("input.toml");
    let mut all_expenses = read_from_file(&filename);
    
    println!("Spend breakdown: ");
    let mut total_spend = 0.0;
    for (person,personal_expenses) in all_expenses.people.iter_mut() {
        println!("Spent by {person}:");
        for(key,value) in personal_expenses.expenses.iter_mut() {
            println!("   {} euros on {}", value, key.replace("_", " "));
            total_spend += *value;
            personal_expenses.personal_total_spend = personal_expenses.personal_total_spend + *value;
        }
        println!("Total spent by {person} is {} euros",personal_expenses.personal_total_spend);
        println!("")
    }

    let per_person_spend = total_spend / (all_expenses.people.len() as f64);
    println!("Total Spend is {},Divided by {}, cost each is {}",total_spend,all_expenses.people.len(),per_person_spend);

    for (person,personal_expenses) in all_expenses.people.iter() {
        let diff = personal_expenses.personal_total_spend - per_person_spend;
        if diff.is_sign_positive() {
            println!("{person} is owed {}",diff);
        }
        else {
            println!("{person} owes {}",-diff);
        }
    }

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