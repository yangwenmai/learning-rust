fn main() {
    let mut bn = 10;
    let result = loop {
        bn = bn + 1;
        println!("again!");
        if bn == 13 {
            break bn * 2;
        }
    };
    println!("result is: {}", result);

    while bn < 30 {
        println!("{}", bn);
        bn += 3;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
    println!("---------");

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    println!("{}", fab(8));
}

// 斐波那契数列: https://zh.wikipedia.org/wiki/%E6%96%90%E6%B3%A2%E9%82%A3%E5%A5%91%E6%95%B0%E5%88%97
fn fab(x: i32) -> i32 {
    if x < 2 {
        return x;
    }
    return fab(x - 1) + fab(x - 2);
}