use hello_world::bill::bills::{Bill, Bills};

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
