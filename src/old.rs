use chrono::naive::NaiveDate;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn main() {
loop {
        print!("> ");
        stdout().flush().unwrap();
        match stdin().lines().next() {
            Some(Ok(input)) => {
                if input.trim() == "exit" {
                    break;
                }
                if input.trim().is_empty() {
                    continue;
                }
                eval("{}", &input);
            }
            _ => {}
        }
    }
}

fn eval(arg: &String) {
    match arg {
        "mortgage" => create_mortgage(),
        "brokerage_account" => create_brokerage_account(),
        "list" => list_entities(),
    }
}

fn create_mortgage() {
    let principal = ask_for_number("How much is your principal?");
    let rate = ask_for_number("What is your rate?");
    let monthly_payment = ask_for_number("What is your monthly payment?");
    let escrow = ask_for_number("What is your escrow payment?");
    let maturity_date = ask_for_date("What is your maturity date?");

}

fn ask_for_date(question: &str) -> chrono::naive::NaiveDate {
    loop {
    print!("{}", question);
    stdout().flush().unwrap();
    match stdin().lines().next() {
        Some(Ok(input)) => {
            if let Ok(value): f32 = chrono::naive::NaiveDate::parse_from_str(input.trim(), "%Y-%m-%d") {
                return value;
            } else {
                println!("Please provide valid date: YYYY-MM-DD");
                continue;
            }
        },
        None => continue;
    }
    }
}


fn ask_for_number(question: &str) -> f32 {
    loop {
    print!(question);
    stdout().flush().unwrap();
    match stdin().lines().next() {
        Some(Ok(input)) => {
            if let Ok(value): f32 = input.trim().parse() {
                return value;
            } else {
                println!("Please provide a number.");
                continue;
            }
        }
    }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_configuration() {
        let example_config = MarketConfiguration::builder()
            .with_expected_annual_return(9)
            .build();

        assert_eq!(example_config.expected_annual_return, Some(9));
    }

    #[test]
    fn test_mortgage_configuration() {
        let example_config = MortgageConfiguration::builder()
            .with_principal(100_000.00)
            .with_rate(7.5)
            .with_escrow_payment(850.50)
            .with_monthly_payment(5100.45)
            .with_maturity_date(NaiveDate::MAX)
            .build();

        assert_eq!(example_config.principal, 100_000_00);
        assert_eq!(example_config.rate, 7.5f32);
        assert_eq!(example_config.escrow_payment, 850_50);
        assert_eq!(example_config.monthly_payment, 5100_45);
        assert_eq!(example_config.maturity_date, NaiveDate::MAX);
    }

    #[test]
    fn test_mortgage_creation() {
        let expected_mortgage: Mortgage = MortgageConfiguration::builder().build().into();

        assert_eq!(expected_mortgage.get_monthly_rate(), 0f32);
    }

    #[test]
    fn test_mortgage_payment_schedule() {
        let conf = MortgageConfiguration::default();
        let mortgage: Mortgage = conf.into();
        let table: AmortizationTable = mortgage.get_amortization_table();
        assert!(table.is_empty());
    }
}

enum Payment {
    MortgagePayment {
        interest: i32,
        principal: i32,
        escrow: i32,
    },
    LoanPayment {
        interest: i32,
        principal: i32,
    },
}

    

struct AmortizationTable {
    payments: Vec<Payment>,
}

impl AmortizationTable {
    fn is_empty(&self) -> bool {
        self.payments.is_empty()
    }
}

#[derive(Debug, Default)]
struct MortgageConfiguration {
    principal: i32,
    rate: f32,
    escrow_payment: i32,
    monthly_payment: i32,
    maturity_date: chrono::naive::NaiveDate,
}

impl MortgageConfiguration {
    fn builder() -> MortgageConfigurationBuilder {
        MortgageConfigurationBuilder::default()
    }
}

#[derive(Debug, Default)]
struct MortgageConfigurationBuilder {
    principal: f32,
    rate: f32,
    escrow_payment: f32,
    monthly_payment: f32,
    maturity_date: chrono::naive::NaiveDate,
}

impl MortgageConfigurationBuilder {
    fn with_principal(mut self, principal: f32) -> Self {
        self.principal = principal;
        self
    }

    fn with_rate(mut self, rate: f32) -> Self {
        self.rate = rate;
        self
    }

    fn with_escrow_payment(mut self, escrow_payment: f32) -> Self {
        self.escrow_payment = escrow_payment;
        self
    }

    fn with_monthly_payment(mut self, monthly_payment: f32) -> Self {
        self.monthly_payment = monthly_payment;
        self
    }

    fn with_maturity_date(mut self, maturity_date: chrono::naive::NaiveDate) -> Self {
        self.maturity_date = maturity_date;
        self
    }

    fn build(self) -> MortgageConfiguration {
        MortgageConfiguration {
            principal: (self.principal * 100f32) as i32,
            rate: self.rate,
            escrow_payment: (self.escrow_payment * 100f32) as i32,
            monthly_payment: (self.monthly_payment * 100f32) as i32,
            maturity_date: self.maturity_date,
        }
    }
}

#[derive(Debug)]
struct MarketConfiguration {
    expected_annual_return: Option<i32>,
}

impl MarketConfiguration {
    fn builder() -> MarketConfigurationBuilder {
        MarketConfigurationBuilder::default()
    }
}

#[derive(Debug, Default)]
struct MarketConfigurationBuilder {
    expected_annual_return: Option<i32>,
}

impl MarketConfigurationBuilder {
    fn with_expected_annual_return(mut self, annual_return: i32) -> Self {
        self.expected_annual_return = Some(annual_return);
        self
    }

    fn build(self) -> MarketConfiguration {
        MarketConfiguration {
            expected_annual_return: self.expected_annual_return,
        }
    }
}

#[derive(Debug, Clone)]
struct Mortgage {
    principal_amount: i32,
    rate: f32,
    escrow_amount: i32,
    monthly_payment: i32,
    maturity_date: chrono::naive::NaiveDate,
}

impl Mortgage {
    fn get_monthly_rate(&self) -> f32 {
        self.rate / 12.0
    }

    fn get_amortization_table(&self) -> AmortizationTable {
        AmortizationTable { payments: Vec::new() }
    }
}

impl From<MortgageConfiguration> for Mortgage {
    fn from(mortgage_configuration: MortgageConfiguration) -> Mortgage {
        Mortgage {
            principal_amount: mortgage_configuration.principal,
            rate: mortgage_configuration.rate,
            escrow_amount: mortgage_configuration.escrow_payment,
            monthly_payment: mortgage_configuration.monthly_payment,
            maturity_date: mortgage_configuration.maturity_date,
        }
    }
}
