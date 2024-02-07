use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    use List::*;
    let value = Rc::new(RefCell::new(10));
    let a = Rc::new(Cons(value.clone(), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(11)), a.clone());
    let c = Cons(Rc::new(RefCell::new(12)), a.clone());

    println!("a: {a:?}");
    println!("b: {b:?}");
    println!("c: {c:?}");
    *value.borrow_mut() += 10;
    println!("----------after change----");
    println!("a: {a:?}");
    println!("b: {b:?}");
    println!("c: {c:?}");
}
