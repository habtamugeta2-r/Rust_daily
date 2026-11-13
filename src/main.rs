fn main() {
    let age = 30;
    // / let tells us new binding is created, i mean the connection between name and value is created, but the value is cannot be changed, it is immutable by default.
    // / so, their relation between name and value is decided by asignment operators,
    // / 
    // / name is representative of the value, and the value is the actual data that is stored in memory, represetative work as the value of the current data.
    // / 
    // / asignment operators are used to create the connection between name and value, and they can be used to change the value of the name, but they cannot be used to change the value of the value.
    // / 
    // / = and == are different, = is used to create the connection between name and value, and == is used to compare the value of the name and the value of the value.
    
    // / the age is immutable by default, so we cannot change the value of age, but we can create a new binding with the same name and a new value, and the new binding will shadow the old binding.
    
    // / age = 31; // This line will cause a compile-time error because `age` is immutable.

    // / but we can create a mutable variable by using the `mut` keyword, which allows us to change the value of the variable after it has been initialized.

    print!("Age is: {}", age); // This will print: Age is: 30
    let mut mutable_age = 30; // This variable is mutable.
    println!("Mutable age is now: {}", mutable_age); // This will print: Mutable age is now: 31
    mutable_age = 31; // This is allowed because `mutable_age` is mutable.
    print!("Mutable age is now: {}", mutable_age); // This will print: Mutable age is now: 31

    // but we thout reasign we can do, take value with new declared variable

    
}
