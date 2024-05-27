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
        if taxable_income <= bracket.min_income {
            break;
        }

        let income_in_bracket = match bracket.max_income {
            Some(max_income) if taxable_income > max_income => max_income - bracket.min_income,
            _ => taxable_income - bracket.min_income,
        };

        tax_due += income_in_bracket * bracket.rate;
        tax_due -= bracket.deduction; // Apply deduction here
    }

    tax_due
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tax_data::TaxBracket;

    #[test]
    fn test_calculate_tax_with_no_income() {
        let user_input = UserInput {
            annual_income: 0.0,
            deductions: 0.0,
            tax_paid: 0.0,
        };

        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0, deduction: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075, deduction: 2033.28 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15, deduction: 4577.27 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225, deduction: 7953.21 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275, deduction: 10752.02 },
            ],
        };

        let tax_due = calculate_tax_complete(&user_input, &tax_brackets);
        assert!((tax_due - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_tax_with_income_under_first_bracket() {
        let user_input = UserInput {
            annual_income: 20000.0,
            deductions: 0.0,
            tax_paid: 0.0,
        };

        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0, deduction: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075, deduction: 2033.28 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15, deduction: 4577.27 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225, deduction: 7953.21 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275, deduction: 10752.02 },
            ],
        };

        let tax_due = calculate_tax_complete(&user_input, &tax_brackets);
        assert!((tax_due - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_tax_with_income_in_second_bracket() {
        let user_input = UserInput {
            annual_income: 30000.0,
            deductions: 0.0,
            tax_paid: 0.0,
        };

        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0, deduction: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075, deduction: 2033.28 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15, deduction: 4577.27 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225, deduction: 7953.21 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275, deduction: 10752.02 },
            ],
        };

        let tax_due = calculate_tax_complete(&user_input, &tax_brackets);
        let expected_tax_due = (30000.0 - 27110.41) * 0.075 - 2033.28;
        assert!((tax_due - expected_tax_due).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_tax_with_income_in_third_bracket() {
        let user_input = UserInput {
            annual_income: 40000.0,
            deductions: 0.0,
            tax_paid: 0.0,
        };

        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0, deduction: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075, deduction: 2033.28 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15, deduction: 4577.27 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225, deduction: 7953.21 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275, deduction: 10752.02 },
            ],
        };

        let tax_due = calculate_tax_complete(&user_input, &tax_brackets);
        let expected_tax_due = (33919.80 - 27110.41) * 0.075 - 2033.28 +
                            (40000.0 - 33919.81) * 0.15 - 4577.27;
        assert!((tax_due - expected_tax_due).abs() < f64::EPSILON);
    }

    #[test]
    fn test_calculate_tax_with_income_in_fourth_bracket() {
        let user_input = UserInput {
            annual_income: 50000.0,
            deductions: 0.0,
            tax_paid: 0.0,
        };
    
        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0, deduction: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075, deduction: 2033.28 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15, deduction: 4577.27 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225, deduction: 7953.21 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275, deduction: 10752.02 },
            ],
        };
    
        let tax_due = calculate_tax_complete(&user_input, &tax_brackets);
        let expected_tax_due = (33919.80 - 27110.41) * 0.075 - 2033.28 +
                               (45012.60 - 33919.81) * 0.15 - 4577.27 +
                               (50000.0 - 45012.61) * 0.225 - 7953.21;
        assert!((tax_due - expected_tax_due).abs() < f64::EPSILON);
    }    

    #[test]
    fn test_calculate_tax_with_income_in_fifth_bracket() {
        let user_input = UserInput {
            annual_income: 60000.0,
            deductions: 0.0,
            tax_paid: 0.0,
        };
    
        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0, deduction: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075, deduction: 2033.28 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15, deduction: 4577.27 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225, deduction: 7953.21 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275, deduction: 10752.02 },
            ],
        };
    
        let tax_due = calculate_tax_complete(&user_input, &tax_brackets);
        let expected_tax_due = (33919.80 - 27110.41) * 0.075 - 2033.28 +
                               (45012.60 - 33919.81) * 0.15 - 4577.27 +
                               (55976.16 - 45012.61) * 0.225 - 7953.21 +
                               (60000.0 - 55976.17) * 0.275 - 10752.02;
        assert!((tax_due - expected_tax_due).abs() < f64::EPSILON);
    }    

    #[test]
    fn test_calculate_tax_simplified_model() {
        let user_input = UserInput {
            annual_income: 100000.0,
            deductions: 0.0,
            tax_paid: 0.0,
        };
    
        let tax_brackets = TaxBrackets {
            brackets: vec![
                TaxBracket { min_income: 0.0, max_income: Some(27110.40), rate: 0.0, deduction: 0.0 },
                TaxBracket { min_income: 27110.41, max_income: Some(33919.80), rate: 0.075, deduction: 2033.28 },
                TaxBracket { min_income: 33919.81, max_income: Some(45012.60), rate: 0.15, deduction: 4577.27 },
                TaxBracket { min_income: 45012.61, max_income: Some(55976.16), rate: 0.225, deduction: 7953.21 },
                TaxBracket { min_income: 55976.17, max_income: None, rate: 0.275, deduction: 10752.02 },
            ],
        };
    
        let max_deduction = 16754.34;
        let simplified_deduction = (user_input.annual_income * 0.20).min(max_deduction);
        let taxable_income = user_input.annual_income - simplified_deduction;
    
        let expected_tax_due = (27110.40 - 0.0) * 0.0 +
                               (33919.80 - 27110.41) * 0.075 - 2033.28 +
                               (45012.60 - 33919.81) * 0.15 - 4577.27 +
                               (55976.16 - 45012.61) * 0.225 - 7953.21 +
                               (taxable_income - 55976.17) * 0.275 - 10752.02;
    
        let tax_due = calculate_tax_simplified(&user_input, &tax_brackets);
    
        assert!((tax_due - expected_tax_due).abs() < f64::EPSILON);
    }    
}
