#[derive(Debug)]
struct Account {
    account_number: String,
    account_holder_name: String,
    account_balance: f64,
}

impl Account {
    fn create_account(
        account_number: String,
        account_holder_name: String,
        account_balance: f64,
    ) -> Account {
        Account {
            account_number,
            account_holder_name,
            account_balance,
        }
    }

    fn deposit(account: &mut Account, amount: f64) {
        let previous_balance: f64 = account.account_balance;
        account.account_balance = previous_balance + amount;
    }

    fn withdraw(account: &mut Account, amount: f64) {
        let previous_balance: f64 = account.account_balance;
        account.account_balance = previous_balance - amount;
    }

    fn get_balance(account: Account) -> f64 {
        return account.account_balance;
    }
}

fn main() {
    let account_01: Account =
        Account::create_account(String::from("1001"), String::from("Jack"), 100.89);
    let mut account_02: Account =
        Account::create_account(String::from("1002"), String::from("Ben"), 18.09);
    let mut account_03: Account =
        Account::create_account(String::from("1003"), String::from("Mohit"), 1690.00);

    // Add amount
    Account::deposit(&mut account_03, 32.19);
    println!("Account after deposit {:?}", account_03);

    // withdraw Amount
    Account::withdraw(&mut account_02, 10.99);
    println!("Account after withdraw {:?}", account_02);

    //Check Balance
    let balance = Account::get_balance(account_01);
    println!("Account balance {}", balance);
}
