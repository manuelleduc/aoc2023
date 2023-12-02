use std::fs::read_to_string;

fn main() {
    day1_2();
}

fn day1_2() {
    let mut total = 0;
    for line in read_to_string("./inputs/1.1").unwrap().lines() {
        let mut c1 = 0;
        let mut c2 = 0;
        for i in 0..(line.len()) {
            let sub = &line[i..];
            let a = if sub.starts_with("1") {
                1
            } else if sub.starts_with("2") {
                2
            } else if sub.starts_with("3") {
                3
            } else if sub.starts_with("4") {
                4
            } else if sub.starts_with("5") {
                5
            } else if sub.starts_with("6") {
                6
            } else if sub.starts_with("7") {
                7
            } else if sub.starts_with("8") {
                8
            } else if sub.starts_with("9") {
                9
            } else if sub.starts_with("one") {
                1
            } else if sub.starts_with("two") {
                2
            } else if sub.starts_with("three") {
                3
            } else if sub.starts_with("four") {
                4
            } else if sub.starts_with("five") {
                5
            } else if sub.starts_with("six") {
                6
            } else if sub.starts_with("seven") {
                7
            } else if sub.starts_with("eight") {
                8
            } else if sub.starts_with("nine") {
                9
            } else {
                -1
            };
            if a > 0 {
                if c1 == 0 {
                    c1 = a;
                    c2 = a
                } else {
                    c2 = a;
                }
            }
        }
        total = total + c1 * 10 + c2;
    }
    println!("{}", total);
}

fn day1_1() {
    let mut total = 0;
    for line in read_to_string("./inputs/1").unwrap().lines() {
        println!("{}", line);
        let mut c1 = 0;
        let mut c2 = 0;
        for c in line.chars() {
            match c.to_digit(10) {
                Some(n) => {
                    println!("{}", n);
                    if c1 == 0 {
                        c1 = n;
                        c2 = n
                    } else {
                        c2 = n
                    }
                }
                _ => {}
            }
            println!("{} {}", c1, c2);
        }
        let line_value = c1 * 10 + c2;
        println!("{}", line_value);
        total = total + line_value
    }
    println!("{}", total);
}
