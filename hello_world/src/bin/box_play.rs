#[derive(Debug)]
struct Car {
    name: String,
}

pub fn main() {
    let mut box_car = Box::new(Car {
        name: "car1".to_string(),
    });
    let box_car2 = box_car.as_mut();
    box_car2.name = "car2".to_string();

    println!("{:?}", box_car.as_ref())
}
