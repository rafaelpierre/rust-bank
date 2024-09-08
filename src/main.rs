

#[derive(Debug)]
struct Account {
    balance: i32,
    id: u32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            balance: 0,
            id,
            holder
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![]}
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}
fn main() {
    let account = Account::new(
        1,
        String::from("me")
    );

    let bank = Bank::new();
    
    // If we simply do let other_bank = bank, we're effectively
    // "moving" the value to the other_bank variable.
    // This will cause an error in the following code if we try
    // to reference the bank binding.

    let other_bank = &bank;

    println!("{:#?}", bank);
}
