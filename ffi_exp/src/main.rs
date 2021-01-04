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
    // global callback support
    fn register_cb(cb: extern fn(i32)) -> i32;
    fn call_cb();
    // targeted callback support using void pointers
    fn register_objcb(target: *mut DummyObj, cb: extern fn(*mut DummyObj, i32) -> i32);
    fn call_objcb();
    // variadic function test
    fn variadictest(x: i32, ...) -> i32;
    // "nullable pointer optimization", e.g. Option Some/None
    fn register_npocb(cb: fn(Option<fn(i32)>, Option<i32>));
    fn call_npocb();
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
    variadic();
    nullptroptimization();
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

fn variadic() {
    unsafe {
        let m = variadictest(3, 1, 2, 3, 9);
        println!("first test got {}", m);
        let n = variadictest(4, 10, 11, 12, 13);
        println!("second test got {}", n);
    }
}

fn npocb_cb(op: Option<fn(i32)>, i: Option<i32>) {
    match op {
        Some(f) => match i {
            Some(ii) => { println!("executing fxn with arg!"); f(ii) },
            None => println!("have fxn, no arg"),
        },
        None    => println!("no fxn provided"),
    }
}

fn nullptroptimization() {
    // you can use optional to deal with NULLed stuff
    unsafe {
        register_npocb(npocb_cb);
        call_npocb();
    }
}