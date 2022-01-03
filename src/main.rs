pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {

    let mut st = Stepper {
        curr: 0,
        step: 3,
        max: 14,
    };

    loop {
        match st.next() {
            Some(v) => println!("loop, {}", v),
            None => break
        }
    }
    println!("All done -------------------------");

    let mut st2 = Stepper {
        curr: 3,
        step: 4,
        max: 15,
    };
    while let Some(n) = st2.next() {
        println!("while, {}!", n);
    }
    println!("All done -------------------------");

    for i in (Stepper { curr:5, step:10, max:50 }) {
        println!("for loop, {}!", i);
    }
    println!("All done -------------------------");

    let mut n = 0;
    loop {
        n += 1;
        if n > 3 {
            break;
        }
        println!("Hello, world! {}", n);
    }
    println!("All done -------------------------");

    n = 0;
    while n < 3 {
        n += 1;
        println!("Hello, world! {}", n);
    }
    println!("All done -------------------------");

    for i in 0..10 {
        println!("Hello, world! {}", i);
    }
    println!("All done -------------------------");

}
