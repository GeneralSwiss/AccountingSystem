/// An event is something along the lines of:
///   - I got a mortgage
///   - I opened a Brokerage Account
///   - I funded a Brokerage Account
///   - I got paid
///   - I took out a margin loan
///   - I paid my mortgage
///   - I repaid the margin loan
pub enum Event {
    NewMortgage { mortgage: Mortgage },
    NewBrokerageAccount { account_name: String },
    DepositIntoAccount { account_name: String, amount: i32 },
    Income { amount: i32 },
    NewMarginLoan { principal: i32, rate: f32 },
}

pub enum EventType {
    Debit { account_id: uuid::Uuid, amount: u32 },
    Credit { account_id: uuid::Uuid, amount: u32 },
}
