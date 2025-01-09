fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; // This is fine
    let z = &mut x; // This is where the error may be
    *z = 15; //This line might cause an error
    println!("{}", x);
}