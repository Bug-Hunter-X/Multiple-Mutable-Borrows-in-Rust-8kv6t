fn main() {
    let mut x = 5;
    { // Creating a scope to limit the mutable borrow of x.
        let y = &mut x; 
        *y = 10; 
        println!("x (after first mutable borrow) = {}", x);
    }
    { //Creating another scope to limit the mutable borrow of x.
        let z = &mut x;
        *z = 15;
        println!("x (after second mutable borrow) = {}", x);
    }
    println!("x = {}", x);
}