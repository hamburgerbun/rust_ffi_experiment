extern crate libc;

#[link(name = "dumb", kind = "static")]
extern {
    // no inputs
    fn printsomething();
    // input unchanged
    fn printthis(input: *const u8);
    // input actually changes
    fn changefirstfive(input: *mut i32);
    // a static variable we want to mess with
    static mut globaltest: i32;
    // TODO: fn that returns something
    // TODO: fn that does global call back
    // TODO: fn that does callback on a specific object 
}

fn main() {
    println!("begin ffi experiment");
    println!("1. printsomething");
    unsafe {
        printsomething();
    }
    println!("2. print this");
    unsafe {
        printthis("this was to be printed".as_ptr());
    }
    println!("3. change the first five ints");
    let mut intvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    unsafe {
        changefirstfive(intvec.as_mut_ptr());
    }
    for x in intvec.iter() {
        println!("value {} in vec", x);
    }
    unsafe {
        println!("initial globaltest int value : {}", globaltest);
        globaltest = 99i32;
        println!("new globaltest int value : {}", globaltest);
    }
    println!("end ffi experiment");
}
