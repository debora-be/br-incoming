mod tax_data;
mod tax_calculator;

use std::fs::File;
use std::io::{self, BufReader, Write};
use serde_json::from_reader;
use clap::{Command, Arg};
use tax_data::{TaxBrackets, UserInput};
use tax_calculator::{calculate_tax_complete, calculate_tax_simplified};

fn main() {
    let matches = Command::new("Brazilian Income Tax Calculator")
        .version("1.0")
        .author("de")
        .about("Calculates Brazilian Income Tax")
        .arg(Arg::new("interactive")
            .short('i')
            .long("interactive")
            .help("Run in interactive mode")
            .action(clap::ArgAction::SetTrue))
        .get_matches();

    if matches.get_flag("interactive") {
        run_interactive();
    } else {
        println!("Use the --interactive option to run the program in interactive mode.");
    }
}

fn run_interactive() {
    println!("This program calculates your Brazilian Income Tax based on your annual income, total deductions, and total tax paid.");

    let annual_income = prompt_float("Enter your annual income (e.g., 50000.00): ");
    let deductions = prompt_float("Enter your total deductions (e.g., 2000.00): ");
    let tax_paid = prompt_float("Enter the total tax paid during the year (e.g., 5000.00): ");

    let user_input = UserInput {
        annual_income,
        deductions,
        tax_paid,
    };

    let file = File::open("src/tax_data.json").expect("Unable to open file");
    let reader = BufReader::new(file);
    let tax_brackets: TaxBrackets = from_reader(reader).expect("Unable to parse JSON");

    let tax_due_complete = calculate_tax_complete(&user_input, &tax_brackets);
    let tax_due_simplified = calculate_tax_simplified(&user_input, &tax_brackets);

    println!("Total Tax Due (Complete Model): {:.2}", tax_due_complete);
    println!("Total Tax Due (Simplified Model): {:.2}", tax_due_simplified);

    let difference_complete = tax_due_complete - user_input.tax_paid;
    let difference_simplified = tax_due_simplified - user_input.tax_paid;

    println!("Tax Difference (Complete Model): {:.2}", difference_complete);
    println!("Tax Difference (Simplified Model): {:.2}", difference_simplified);

    if difference_complete > 0.0 {
        println!("You need to pay {:.2} more using the Complete Model.", difference_complete);
    } else {
        println!("You will be refunded {:.2} using the Complete Model.", -difference_complete);
    }

    if difference_simplified > 0.0 {
        println!("You need to pay {:.2} more using the Simplified Model.", difference_simplified);
    } else {
        println!("You will be refunded {:.2} using the Simplified Model.", -difference_simplified);
    }
}

fn prompt_float(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}
