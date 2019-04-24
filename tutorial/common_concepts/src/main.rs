/*
* This is a variables, data types and function practive.
* @yangwenmai
*/
const MAX_POINTS: u32 = 100_000;
// const INDEX: usize = 10;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}, max_points: {}", x, MAX_POINTS);

    let y = 6;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();

    //  spaces = spaces.len();

    println!("spaces is: {}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess is: {}", guess);

    let z = 0.0;
    let z2: f32 = 3.333;
    println!("z is {}, z is {}", z, z2);

    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let _remainder = 43 % 5;
    println!(
        "sum is {}, diffence is {}, product is {}, quotient is:{}",
        sum, difference, product, quotient
    );

    // 布尔类型
    let f = false;
    let f2: bool = true;
    println!("f is: {}, f2 is: {}", f, f2);
    t();

    // 字符类型
    let c = 'z';
    let c = 'Z';
    let heart_eyed_cat = '🐱';
    println!("c is: {}, cat is: {}", c, heart_eyed_cat);

    // 复核类型-元组类型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup is: {:?}", tup);
    println!("tup is: {:#?}", tup);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let first = tup.0;
    let mid = tup.1;
    let last = tup.2;
    println!("first is: {}, mid is: {}, last is: {}", first, mid, last);

    // 复核类型-数组类型
    let a = [1, 2, 3, 5, 8, 4, 6];
    println!("a[0] is: {}", a[0]);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a: [i32; 5] = [1, 3, 4, 5, 6];
    println!("a is: {:?}, months is: {:#?}", a, months);
    // // 可以正确编译： cargo build
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);
    // // 编译报错：cargo build
    // println!("a[0] is: {}, a[6] is: {}", a[0], a[6]);// error: index out of bounds: the len is 5 but the index is 6
    // let element = a[10];
    // println!("The value of element is: {}", element);

    // let element = a[INDEX]; // error: index out of bounds: the len is 5 but the index is 10
    // println!("The value of element is: {}", element);
    // 函数
    t2(x, "2a".to_string());

    // 语句和表达式的函数体
    let y = {
        let x = 3;
        x + 1
    };
    println!("y is: {}", y);

    let r = t2(3, "3a".to_string());
    println!("r is: {}", r);

    let r = t3(1);
    println!("r is: {}", r);

}

fn t() {
    let f = false;
    let f2: bool = true;
    println!("f is: {}, f2 is: {}", f, f2)
}

fn t2(x: i32, y: String) -> i32 {
    println!("x is: {}, y is: {}", x, y);
    22
}

fn t3(x: i32) -> i32 {
    x + 1
}
