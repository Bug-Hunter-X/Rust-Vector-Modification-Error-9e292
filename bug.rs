fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 42; //Modifying the first element
    }
    println!("Modified vector: {:?}", v);
    //Error: v is deallocated already
    //Use &mut v to avoid this error
}