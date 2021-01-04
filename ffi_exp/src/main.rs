extern crate libc;

#[repr(C)]
struct DummyObj {
    a: i32,
}

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
    fn register_cb(cb: extern fn(i32)) -> i32;
    fn call_cb();
    // TODO: fn that does callback on a specific object 
    fn register_objcb(target: *mut DummyObj, cb: extern fn(*mut DummyObj, i32) -> i32);
    fn call_objcb();
}


extern "C" fn objcallback(target: *mut DummyObj, a: i32) -> i32 {
    println!("called from co to update a thing to {}", a);
    unsafe {
        (*target).a = a;
    }
    a
}

extern fn globalcallback(a: i32) {
    println!("lol got called with value {}", a)
}

fn main() {
    println!("begin ffi experiment");
    justprint(); 
    printwithinput();
    modifyvec(); 
    globalvaluetest();
    globalrustcallback(); 
    objectrustcallback();
    println!("end ffi experiment");
}

fn justprint() {
    println!("1. printsomething");
    unsafe {
        printsomething();
    }
}

fn printwithinput() {
    println!("2. print this");
    unsafe {
        printthis("this was to be printed".as_ptr());
    }
}

fn modifyvec() {
    println!("3. change the first five ints");
    let mut intvec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    unsafe {
        changefirstfive(intvec.as_mut_ptr());
    }
    for x in intvec.iter() {
        println!("value {} in vec", x);
    }
}

fn globalvaluetest() {
    unsafe {
        println!("initial globaltest int value : {}", globaltest);
        globaltest = 99i32;
        println!("new globaltest int value : {}", globaltest);
    }
}

fn globalrustcallback() {
    println!("globalcallback stuff start");
    unsafe {
        let retval = register_cb(globalcallback);
        println!("retval from register was {}", retval);
        call_cb();
    }
    println!("globalcallback stuff end");
}

fn objectrustcallback() {
    println!("objectrustcallback start");
    let mut dummyobj = Box::new(DummyObj{a: 0i32});
    unsafe {
        register_objcb(&mut *dummyobj, objcallback);
        call_objcb();
    }
    println!("objectrustcallback end");
}