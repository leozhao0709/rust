use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct A(Rc<RefCell<Vec<i32>>>);
#[derive(Debug)]
struct B(Rc<RefCell<Vec<i32>>>);
#[derive(Debug)]
struct C(Rc<RefCell<Vec<i32>>>);

fn main() {
    let v = Rc::new(RefCell::new(vec![1, 2, 3]));
    let a = A(v.clone());
    let b = B(v.clone());
    let c = C(v.clone());

    drop(v);

    println!("a is {a:?}");
    a.0.borrow_mut().push(4);
    // drop(a);
    println!("b is {b:?}");
    b.0.borrow_mut().push(5);
    println!("c is {c:?}");
    println!("a is {a:?}");
    println!("b is {b:?}");
}

// #[derive(Debug)]
// struct A(Vec<i32>);
// #[derive(Debug)]
// struct B(Vec<i32>);
// #[derive(Debug)]
// struct C(Vec<i32>);

// fn main() {
//     let v = vec![1, 2, 3];
//     let mut a = A(v.clone());
//     let mut b = B(v.clone());
//     let c = C(v.clone());

//     drop(v);

//     println!("a is {a:?}");
//     a.0.push(4);
//     // drop(a);
//     println!("b is {b:?}");
//     b.0.push(5);
//     println!("c is {c:?}");
//     println!("a is {a:?}");
//     println!("b is {b:?}");
// }
