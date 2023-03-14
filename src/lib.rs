use std::cell::{RefCell, RefMut};

#[derive(Debug, Clone)]
pub struct BankAccount {
     id: u32,
     balance: f64,
     owner: String,
}

impl BankAccount {
    pub fn new(owner: &str) -> Self {
        BankAccount {
            id: 0,
            balance: 0.0,
            owner: owner.to_string()
        }
    }

    pub fn deposit(&mut self, amount: f64){
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), String>{
        if self.balance <= 0 as f64 {
            return Err(String::from("Not enough money"))
        }
        self.balance -= amount;
        Ok(())
    }

    pub fn transfer(&mut self, amount: f64, to_account: &mut RefMut<BankAccount> ) {
        self.withdraw(amount).expect("Something went wrong");
        to_account.deposit(amount);
    }
}


#[derive(Debug, Clone)]
pub struct Bank {
    pub banks: Vec<RefCell<BankAccount>>
}

impl Bank {
    pub fn new(banks: Vec<RefCell<BankAccount>>) -> Self {
        Bank {
            banks
        }
    }

    pub fn add_account(&mut self, owner: &str){
        self.banks.push(RefCell::new(BankAccount::new(owner)));
    }

    pub fn delete_account(&mut self, owner: &str) -> Result<(), String> {
        if self.banks.is_empty(){
            return Err(String::from("It seems there are no banks to delete"))
        }

       if let Some(i) =  self.banks.iter().position(|x| x.borrow_mut().owner == owner) {
           self.banks.remove(i);
       }
        Ok(())
    }
}