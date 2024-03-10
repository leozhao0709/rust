#[derive(Debug, Clone)]
pub struct Bill {
    pub name: String,
    pub amount: f64,
}
pub struct Bills {
    pub all: Vec<Bill>,
}

impl Bills {
    pub fn new() -> Self {
        Self { all: vec![] }
    }

    pub fn add_bill(&mut self, bill: Bill) {
        self.all.push(bill)
    }

    pub fn get_all(&self) -> Vec<&Bill> {
        self.all.iter().collect()
    }
}
