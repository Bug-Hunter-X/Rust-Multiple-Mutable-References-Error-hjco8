fn main() {
    let mut x = 5;
    { //This is the solution with a scope
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 15; 
    println!("{}", x);
}