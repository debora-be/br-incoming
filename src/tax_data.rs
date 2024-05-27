use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxBrackets {
    pub brackets: Vec<TaxBracket>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaxBracket {
    pub min_income: f64,
    pub max_income: Option<f64>,
    pub rate: f64,
    pub deduction: f64,
}

#[derive(Debug)]
pub struct UserInput {
    pub annual_income: f64,
    pub deductions: f64,
    pub tax_paid: f64,
}
