use hello_world::bill::bills::{Bill, Bills};

mod person {
    use hello_world::bill::bills::Bill;
    pub struct Person {
        name: String,
        bill: Bill,
    }

    impl Person {
        pub fn new(name: String, bill: Bill) -> Self {
            Person { name, bill }
        }

        pub fn get_bill(&self) -> Bill {
            self.bill.clone()
        }
    }

    impl Default for Person {
        fn default() -> Self {
            Self {
                name: Default::default(),
                bill: Bill {
                    name: "name".to_owned(),
                    amount: 20.0,
                },
            }
        }
    }
}

pub fn main() {
    let bill = Bill {
        name: "bill1".to_owned(),
        amount: 64.0,
    };
    let p1 = person::Person::new("lei".to_owned(), bill.clone());
    p1.get_bill();

    let mut bills = Bills::new();
    bills.add_bill(bill);

    bills
        .get_all()
        .iter()
        .for_each(|&bill| println!("{bill:?}"))
}
