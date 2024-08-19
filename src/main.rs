mod zroot5;

use num_bigint::BigInt;

use crate::zroot5::ZRoot5;

pub fn fib(n: u32) -> BigInt {
    let a = ZRoot5::new(1, 1, 1);
    // let b = ZRoot5::new(1, -1, 1);
    // println!("{:?}", a.clone() * &b);

    // for i in 0..n {}
    let f = a.pow(n);
    // f = f.clone() + ZRoot5::new(-f.a, f.b, f.n);
    // println
    // println!("{:?}", f);
    // todo!()
    // println!("{:?}", f);
    // f.b
    if f.n == 0 {
        f.b * 2
    } else {
        f.b >> (f.n - 1)
    }
}

fn main() {
    for _ in 0..100 {
        for i in 0..50_000 {
            let start = std::time::Instant::now();
            let x = fib(i * 1000);
            let diff = start.elapsed();
            let x = x.to_string();
            println!(
                "{:?},{:?}",
                i * 1000,
                diff.as_micros(),
                // x // x[0..10.min(x.len())].to_string()
            );
            // println!("{:?}", start.elapsed());
        }
    }
    // let a = ZRoot5::new(1, 1, 1);
    // let b = ZRoot5::new(1, -1, 1);
    // for i in 0..10 {
    //     println!("{:?} {:?}", a.pow(i), b.pow(i));
    // }
    // }
    // let x = fib(7_000_000);
    // println!("{}", fib(1_000_000));
    // println!("Hello, world!");
}
