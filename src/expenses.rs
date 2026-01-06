/**
 * 	This class handles reading the expenses from a file to a struct, and any necessary currency conversions
 */
use std::collections::HashMap;
use toml::Table;
use toml::Value;
use std::fs;

use crate::main;

 // Array of involved poeple, each with a set of expenses
#[derive(Clone)]
pub struct Expenses {
    people: HashMap<String,PersonalExpenses>,
	//currency list in the format {"£":1,"K": 0.074}, Where £ is the output currency and 1 Krone = £0.074
	currencies: HashMap<String,f64>,
}

// A set of expenses, plus a field to total all the expenses
#[derive(Clone)]
struct PersonalExpenses {
	//Expenses in the format {"name_of_expense","£100"}
    expenses: Vec<Expense>,
	//Total spend, converted to main currency
    personal_total_spend: f64,	
}

#[derive(Clone)]
struct Expense {
    name: String,
	amount: f64,
	currency: char
}

impl Expenses {
	pub fn new() -> Expenses {
		 Expenses {
			people: HashMap::new(),
			currencies: HashMap::new(),
		}
	}

	pub fn read_from_file(filename: &String) -> Expenses {
		let input = match fs::read_to_string(filename).expect("Error opening file {filename}");
		let data: Table = match toml::from_str(input.as_str())
		{
			Ok(value) => value,
			Err(error) => panic!("Could not parse toml: {error}"),
		};
		
		let main_currency: char = '£';

		for (key, value) in data.iter() {
			match value {
				Value::Table(table) => {
					let mut personal_expenses = PersonalExpenses { expenses: Vec::new(),personal_total_spend: 0.0 };

					for (key, value) in table{
						let mut expense = Expense {name: key.clone(),amount: 0.0,currency: ' '};

						if let Value::Float(val) = value {
							expense.amount = val.clone();
							expense.currency = main_currency;
						}
						else if let Value::Integer(val) = value {
							expense.amount = val.clone() as f64;
							expense.currency = main_currency;
						}
						else if let Value::String(val) = value {
							expense.amount = val[1:].clone();
							expense.currency = val[0];
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

	pub fn print_spend_breakdown() {
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
	}
}


