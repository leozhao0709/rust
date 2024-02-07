fn main() {
    let x = 0;

    let res = return_op(x).unwrap_or_else(|err_str| format!("error: {err_str}"));
    // let res = return_op(x).expect("error");
    println!("{}", res);
}

fn return_op(x: i8) -> Result<String, String> {
    if x == 0 {
        Err("x should not be 0!".to_owned())
    } else {
        Ok(format!("x is {x}"))
    }
}
