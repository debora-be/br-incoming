use crate::tax_data::{TaxBrackets, UserInput};

pub fn calculate_tax_complete(user_input: &UserInput, tax_brackets: &TaxBrackets) -> f64 {
    let taxable_income = user_input.annual_income - user_input.deductions;
    calculate_tax(taxable_income, &tax_brackets)
}

pub fn calculate_tax_simplified(user_input: &UserInput, tax_brackets: &TaxBrackets) -> f64 {
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
            }
        } else {
            if taxable_income > bracket.min_income {
                let income_in_bracket = taxable_income - bracket.min_income;
                tax_due += income_in_bracket * bracket.rate;
            }
        }
    }

    tax_due
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tax_complete() {
        let user_input = UserInput {
            annual_income: 50000.0,
            deductions: 2000.0,
            tax_paid: 0.0,
        };

        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275 },
            ],
        };

        let tax_due = calculate_tax_complete(&user_input, &tax_brackets);
        assert!((tax_due - 6700.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_tax_simplified() {
        let user_input = UserInput {
            annual_income: 50000.0,
            deductions: 0.0,
            tax_paid: 0.0,
        };

        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275 },
            ],
        };

        let tax_due = calculate_tax_simplified(&user_input, &tax_brackets);
        assert!((tax_due - 6879.132).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_tax_with_no_income() {
        let user_input = UserInput {
            annual_income: 0.0,
            deductions: 0.0,
            tax_paid: 0.0,
        };

        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275 },
            ],
        };

        let tax_due = calculate_tax_complete(&user_input, &tax_brackets);
        assert!((tax_due - 0.0).abs() < f64::EPSILON);
    }
}
