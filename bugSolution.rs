fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = &mut v; //Using &mut v avoids the error
    unsafe {
        *ptr.get_unchecked(0) = 42; //Modifying the first element
    }
    println!("Modified vector: {:?}", v);
}