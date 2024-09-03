pub struct Transaction {
    transaction_type: TransactionType,
}

enum TransactionType {
    Debit,
    Credit,
}
