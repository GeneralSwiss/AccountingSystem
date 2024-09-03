use crate::event::{Event, EventProcessor, EventType};
use crate::transaction::Transaction;

pub struct Account {
    pub id: AccountId,
    name: String,
    account_type: AccountType,
    transactions: Vec<Transaction>,
    event_processor: EventProcessor,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct AccountId(uuid::Uuid);

#[derive(Debug, PartialEq, Eq)]
enum AccountType {
    Liability,
    Asset,
    Expense,
    Income,
}

impl Account {
    pub fn new(name: &str, account_type: AccountType, event_processor: EventProcessor) -> Self {
        Account {
            id: AccountId(uuid::Uuid::new_v4()),
            name: name.to_string(),
            account_type,
            transactions: Vec::new(),
            event_processor,
        }
    }
    pub fn process_event(&mut self, event: &Event) {}

    pub fn generate_event(&mut self) -> Vec<Event> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let sut = Account::new("TestAccount", AccountType::Asset, EventProcessor::NoOp);

        assert_eq!(&sut.name, "TestAccount");
        assert_eq!(sut.account_type, AccountType::Asset);
        assert!(sut.transactions.is_empty());
    }

    #[test]
    fn test_process_event() {
        let account = Account::new("Test", AccountType::Revenue, EventProcessor::NoOp);
        let sample_event = Event::GetPaid(9400.00);
        account.process_event(&sample_event)
    }
}
