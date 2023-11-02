use std::io::Error;
use std::ptr::null;

fn main() {
    let bank1 = BankAccount{
        account_number: "123".to_string(),
        holder_name: "Alice".to_string(),
        balance: 100.0,
    };

    let bank2 = BankAccount{
        account_number: "456".to_string(),
        holder_name: "Bob".to_string(),
        balance: 120.0,
    };

    let bank3 = BankAccount{
        account_number: "789".to_string(),
        holder_name: "John".to_string(),
        balance: 330.0,
    };

    let mut accounts: Vec<Box<BankAccount>> = Vec::new();
    accounts.push(Box::new(bank1));
    accounts.push(Box::new(bank2));
    accounts.push(Box::new(bank3));


    for mut account in accounts {
        let deposit = account.deposit();
        let withdraw = account.withdraw(10.0);
        let balance = account.get_balance();

        match deposit {
            Ok(msg) => println!("Deposit: {}", msg),
            Err(err) => panic!("Deposit Error! {}", err)
        }

        match withdraw {
            Ok(msg) => println!("Withdraw: {}", msg),
            Err(err) => panic!("Withdraw Error! {}", err)
        }

        match balance {
            Ok(msg) => println!("Balance: {}", msg),
            Err(err) => panic!("Balance Error! {}", err)
        }
    }

}

trait Account {
    type BankAccount;
    fn deposit(&self) -> Result<String, String>;
    fn withdraw(&mut self, amount: f32) -> Result<f32, String>;
    fn get_balance(&self) -> Result<f32, String>;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f32,
}

impl Account for BankAccount {
    type BankAccount = ();

    fn deposit(&self) -> Result<String, String> {
        if self.account_number == "" {
            return Err("Invalid Account!".to_string());
        }
        Ok(self.account_number.to_string())
    }

    fn withdraw(&mut self, amount: f32) -> Result<f32, String> {
        if self.account_number == "" {
            return Err("Invalid Account!".to_string());
        }

        if self.balance < 0.0 {
            return Err("Invalid account balance!".to_string());
        }

        self.balance -= amount;
        Ok(self.balance)
    }

    fn get_balance(&self) -> Result<f32, String> {
        if self.account_number == "" {
            return Err("Invalid Account!".to_string());
        }

        Ok(self.balance)
    }
}


