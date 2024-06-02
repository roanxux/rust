const MAX_NUM: i32 = 16;

static SAFE: i32 = 32;

static mut UNSAFE: i32 = 32;


fn define_x() -> &'static str {
   let x:&str = "hello";
   return x;
}

fn main() {
    const MIN_NUM: i32 = 100;
    let x: &'static str = define_x();
    let _s = 333;
    println!("{}, world\n", x);
    print!("min_mux is {MIN_NUM}\n");
    print!("max_mux is {MAX_NUM}\n");
    println!("i8 is {} byte", std::mem::size_of::<i8>());
    println!("i16 is {} byte", std::mem::size_of::<i16>());
    println!("i32 is {} byte", std::mem::size_of::<i32>());
    println!("i64 is {} byte", std::mem::size_of::<i64>());
    println!("u8 is {} byte", std::mem::size_of::<u8>());
    println!("u16 is {} byte", std::mem::size_of::<u16>());
    println!("u32 is {} byte", std::mem::size_of::<u32>());
    println!("u64 is {} byte", std::mem::size_of::<u64>());
    println!("usize is {} byte", std::mem::size_of::<usize>());
    println!("isize is {} byte", std::mem::size_of::<isize>());

    //static
    println!("safe is {SAFE}");

    unsafe{
        println!("unsafe is {UNSAFE}");
        UNSAFE = 48;
        println!("update unsafe is {UNSAFE}");
    }

    //float
    let f1: f32 = 8.3333;
    let f2: f64 = 9.3333;

    println!("f1 {:.2} {:.3} {:.4}", f1, f1, f1);
    println!("f2 {:.2} {:.3} {:.4}", f2, f2, f2);

    //数组
    let arry = [11, 12, 13, 14];

    println!("arry {:?}", arry);

    let mut arry1 = [11, 12, 13, 14];

    println!("arry1 {:?}", arry1);

    arry1 = [arry[0]+1, arry[1]+1, arry[2]+1, arry[3]+1];
    println!("update arry1 {:?}", arry1);
    println!("arry1 len is {}", arry1.len());

    for i in arry1 {
        println!("arry1 is {}", i);
    }

    let arr = [0; 3];

    for i in arr {
        println!("arr is {}", i);
    }

    //元组
    let tup: (i32, &str, f64) =(0, "xux", 3.14);
    println!("tup is {}, {}, {}", tup.0, tup.1, tup.2);

    let mut tup1: (i32, &str, f64) =(0, "xux", 3.14);

    println!("tup1 is {}, {}, {}", tup1.0, tup1.1, tup1.2);
    tup1.0 = 32;
    tup1.1 = "xuxing";
    tup1.2 = 4.857;

    println!("update tup1 is {}, {}, {}", tup1.0, tup1.1, tup1.2);

    let tup3 =();

    println!("tup3 {:?} ", tup3);


    //onwership
    
}