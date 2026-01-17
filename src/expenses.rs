/**
 * 	This class handles reading the expenses from a file to a struct, and any necessary currency conversions
 */
use std::collections::HashMap;
use std::str::FromStr;
use toml::Table;
use toml::Value;
use std::fs;
use std::fmt;

 // Array of involved poeple, each with a set of expenses
#[derive(Clone)]
#[derive(Debug)]
pub struct Expenses {
	// Array of people, with how much each person spent, in main_currency units
    pub people: Vec<String>,
	pub expenses: Vec<Expense>,
	pub main_currency: char,
	//currency list in the format {"K": 0.074}, Where £ is the output currency and 1 Krone = £0.074
	pub currencies: HashMap<String,f64>,
	//Weighting for people's payments, useful for joint bank accounts for couples
	pub weights: HashMap<String,f64>,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Expense {
    pub name: String,
	pub amount: f64,
	pub currency: char,
	pub brought_by: Vec<String>, //This expense was brought by who, should never be empty
	pub expense_for: Vec<String> //This expense was brought for who, empty array means equally split between everyone
}

impl Expense {
	pub fn new() -> Expense {
		 Expense {
			name: String::new(),
			amount: 0.0,
			currency: '£',
			brought_by: Vec::new(),
			expense_for: Vec::new()
		}
	}
}
impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut brought_by_str = String::new();
		let mut expense_for_str = String::new();

		for name in self.brought_by.iter() {
			brought_by_str.push_str(name);
			brought_by_str.push_str(",");
		}
		brought_by_str = brought_by_str[..brought_by_str.len()-1].to_string();

		for name in self.expense_for.iter() {
			expense_for_str.push_str(name);
			expense_for_str.push_str(",");
		}
		if expense_for_str.len() == 0 {
			expense_for_str = "everyone".to_string();
		}
		else {
			expense_for_str = expense_for_str[..expense_for_str.len()-1].to_string();
		}
		
        write!(f, "{} Spent {}{} on {} for {}", brought_by_str, self.currency,self.amount,self.name,expense_for_str)
    }
}

impl Expenses {
	pub fn new() -> Expenses {
		 Expenses {
			people: Vec::new(),
			expenses: Vec::new(),
			currencies: HashMap::new(),
			main_currency: '£',
			weights: HashMap::new()
		}
	}

	pub fn read_from_file(&mut self,filename: String) {
		let input = fs::read_to_string(filename).expect("Error opening file {filename}");
		let data: Table = match toml::from_str(input.as_str())
		{                                 
			Ok(value) => value,
			Err(error) => panic!("Could not parse toml: {error}"),
		};
		
		for (key, value) in data.iter() {
			match value {
				Value::Table(table) => {
					match key.as_str() {
						"Currency" => self.parse_currencies(table),
						"Weights" => self.parse_weights(table),
						_ => self.parse_expenses(key,table)
					}
					
				}
				_ => println!("First level should only be name labels, got: {}", value),
			}
		}
	}

	fn parse_currencies(&mut self,table: &toml::map::Map<String, Value>)
	{
		for (key, value) in table {
			if let Value::String(val) = value {
				if key.as_str() == "main" {
					self.main_currency = val.chars().nth(0).unwrap();
				}
				else {
					let compare_currency = val.chars().nth(0).unwrap();
					let temp = val.replace(compare_currency, "");
					if compare_currency != self.main_currency {
						println!("Invalid Currency, always compare to main currency");
						continue;
					}
					self.currencies.insert(key.clone(), temp.parse().unwrap());
				}
			}
		}
		//println!("Main Currency: {}",self.main_currency);
		//for (key, value) in self.currencies.iter_mut() {
		//	println!("Other Currency: {key} = £{value}",);
		//}
		//println!();
	}
	fn parse_weights(&mut self, table: &toml::map::Map<String, Value>) {
		for (key, value) in table {
			if let Value::Float(val) = value {
				self.weights.insert(key.clone(), val.clone() as f64);
			}
			else if let Value::Integer(val) = value {
				self.weights.insert(key.clone(), val.clone() as f64);
			}
		}
	}

	fn parse_expenses(&mut self,names: &String, table: &toml::map::Map<String, Value>)
	{
		// Get Buyers and Brought for lists from the Table Header name
		let mut i: usize = 0;
		let mut buyers: Vec<String> = Vec::new();
		let mut brought_for: Vec<String> = Vec::new();

		// The format here is List_Of_Buyers-List_Of_Brought_for
		for names in names.split("-") {
			let mut names_array: Vec<String> = Vec::new();
			for name_str in names.split("_") {
				let name = String::from_str(name_str).unwrap();
				if !self.people.contains(&name) {
					self.people.push(name.clone());
				}
				names_array.push(name);
			}
			
			if i == 0 {
				buyers.append(&mut names_array);
			}
			else if i == 1 {
				brought_for.append(&mut names_array);
			}
			i+=1;
		}
		if buyers.len() == 0 {
			println!("No buyers detected for expense group: {names}, skipping...");
			return;
		}
		
		//Loop through the list of expenses for this group
		for (key, value) in table{
			let mut expense = Expense::new();
			expense.name = key.clone();
			expense.brought_by = buyers.clone();
			expense.expense_for = brought_for.clone();

			if let Value::Float(val) = value {
				expense.amount = val.clone();
				expense.currency = self.main_currency;
			}
			else if let Value::Integer(val) = value {
				expense.amount = val.clone() as f64;
				expense.currency = self.main_currency;
			}
			else if let Value::String(val) = value {
				expense.currency = val.chars().nth(0).unwrap();
				let temp = val.replace(expense.currency, "");
				expense.amount = temp.parse().unwrap();
				
			}
			else {
				println!("Got invalid value on second level {}",value);
			}

			self.expenses.push(expense);
		}
		
	}

	pub fn total_spend(&mut self) -> HashMap<String,f64>{
		let mut spend: HashMap<String,f64> = HashMap::new();

		for expense in self.expenses.iter() {
			//Calculate Amount of money
			let num_buyers = expense.brought_by.len() as f64;
			let mut amount = expense.amount;
			if expense.currency != self.main_currency {
				let key: String = expense.currency.to_string();
				let conversion: f64 = self.currencies.get(&key).unwrap().clone();
				amount *= conversion;
			}
			amount /= num_buyers;

			for buyer in expense.brought_by.iter() {
				*spend.entry(buyer.clone()).or_insert(0.0) += amount;
			}
		}
		spend
	}

}


