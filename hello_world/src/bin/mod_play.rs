mod private_module {
    pub fn public_function() {
        println!("This function is public within a private module.");
    }

    fn private_function() {
        println!("This function is private to private_module.");
    }
}

fn main() {
    // Error: `private_module` is private
    // private_module::public_function();

    // Also, using `use` will result in an error
    // use private_module::public_function;
    // public_function();

    let i = 5;
    println!("{}", i / 2);
}
