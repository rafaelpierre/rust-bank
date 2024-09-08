

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

    print_account(&account);
    print_account(&account);
    println!("{:#?}", bank);
}
