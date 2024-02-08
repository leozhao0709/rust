use hello_world::bill::bills::{Bill, Bills};

struct Person {
    name: String,
    bill: Bill,
}

impl Person {
    fn new(name: String, bill: Bill) -> Self {
        Person { name, bill }
    }

    fn get_bill(&self) -> Bill {
        self.bill.clone()
    }
}

pub fn main() {
    let bill = Bill {
        name: "bill1".to_owned(),
        amount: 64.0,
    };
    let mut bills = Bills::new();
    bills.add_bill(bill);

    bills
        .get_all()
        .iter()
        .for_each(|&bill| println!("{bill:?}"))
}
