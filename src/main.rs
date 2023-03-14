use std::cell::{Ref, RefCell};
use bank::{Bank, BankAccount};

fn main() {
    let mut container = Bank {
        banks: Vec::new(),
    };

    container.add_account("Sohib");
    container.add_account("Rasul");
    println!("Here is the added bank: {:?}", &container);

    let sohib = &mut container.banks[0].borrow_mut();

    sohib.deposit(2500.0);
    println!("Here is the updated bank: {:?}", &container);

    let rasul =  &mut container.banks[1].borrow_mut();

    sohib.transfer(1500.0, rasul);
    println!("Here is the updated bank: {:?}", &container);

    container.delete_account("Sohib").unwrap();
    println!("Here is the removed bank: {:?}", &container);
}
