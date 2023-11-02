fn task() {
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
        let balance = account.balance();

        println!("Deposit: {}", deposit);
        println!("Withdraw: {}", withdraw);
        println!("Balance: {}", balance);
    }

}

trait Account {
    type BankAccount;
    fn deposit(&self) -> String;
    fn withdraw(&mut self, amount: f32) -> f32;
    fn balance(&self) -> f32;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f32,
}

impl Account for BankAccount {
    type BankAccount = ();

    fn deposit(&self) -> String {
        self.account_number.to_string()
    }

    fn withdraw(&mut self, amount: f32) -> f32 {
        self.balance -= amount;
        self.balance
    }

    fn balance(&self) -> f32 {
        self.balance
    }
}


