mod tax_data;

use std::fs::File;
use std::io::BufReader;
use serde_json::from_reader;
use clap::{Command, Arg};
use tax_data::{TaxBrackets, UserInput};

fn main() {
    let matches = Command::new("Brazilian Income Tax Calculator")
        .version("1.0")
        .author("Your Name")
        .about("Calculates Brazilian Income Tax")
        .arg(Arg::new("income")
            .help("Annual income")
            .required(true)
            .value_parser(clap::value_parser!(f64)))
        .arg(Arg::new("deductions")
            .help("Total deductions")
            .required(true)
            .value_parser(clap::value_parser!(f64)))
        .arg(Arg::new("tax_paid")
            .help("Total tax paid during the year")
            .required(true)
            .value_parser(clap::value_parser!(f64)))
        .get_matches();

    let annual_income: f64 = *matches.get_one::<f64>("income").expect("Invalid income");
    let deductions: f64 = *matches.get_one::<f64>("deductions").expect("Invalid deductions");
    let tax_paid: f64 = *matches.get_one::<f64>("tax_paid").expect("Invalid tax paid");

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

fn calculate_tax_complete(user_input: &UserInput, tax_brackets: &TaxBrackets) -> f64 {
    let taxable_income = user_input.annual_income - user_input.deductions;
    calculate_tax(taxable_income, &tax_brackets)
}

fn calculate_tax_simplified(user_input: &UserInput, tax_brackets: &TaxBrackets) -> f64 {
    let max_deduction = 16754.34;
    let simplified_deduction = (user_input.annual_income * 0.20).min(max_deduction);
    let taxable_income = user_input.annual_income - simplified_deduction;
    calculate_tax(taxable_income, &tax_brackets)
}

fn calculate_tax(taxable_income: f64, tax_brackets: &TaxBrackets) -> f64 {
    let mut tax_due = 0.0;

    for bracket in &tax_brackets.brackets {
        if let Some(max_income) = bracket.max_income {
            if taxable_income > bracket.min_income {
                let income_in_bracket = (taxable_income - bracket.min_income).min(max_income - bracket.min_income);
                tax_due += income_in_bracket * bracket.rate;
                tax_due -= bracket.deduction;
            }
        } else {
            if taxable_income > bracket.min_income {
                let income_in_bracket = taxable_income - bracket.min_income;
                tax_due += income_in_bracket * bracket.rate;
                tax_due -= bracket.deduction;
            }
        }
    }

    tax_due
}
