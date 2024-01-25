use async_graphql::*;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum LoanStatus {
    Applied,
    Approved,
    Disbursed,
    Repaid,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
    Charge,
    Refund,
    SendMoney,
    ReceiveMoney,
    PayBill,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Authorized,                                                                                                                                                                                            
    Captured,                                                                                                                                                                                              
    Refunded,                                                                                                                                                                                              
    Initiated, 
}

