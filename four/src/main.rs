fn main() {
    let min = 100;
    let max = 999;

    for i in ((min * min)..(max * max)).rev() {
        if let Some((m1, m2)) = check(i, max, min) {
            println!("{} x {} = {}", m1, m2, i);
            return;
        }
    }
}

fn int_len_base_10(n: i32) -> i32 {
    let mut n = n;
    let mut len = 0;
    loop {
        n = n / 10;
        len += 1;
        if n == 0 {
            break;
        }
    }

    len
}

fn check(num: i32, max_d: i32, min_d: i32) -> Option<(i32, i32)> {
    let mut vec: Vec<i32> = Vec::new();
    let mut n = num;

    loop {
        vec.push(n % 10);
        n = n / 10;

        if(n == 0) {
            break;
        }
    }

    for i in 0..(vec.len() / 2) {
        if vec[i] != vec[vec.len() - 1 - i] {
            return None;
        }
    }

    for i in (min_d..=max_d).rev() {
        if num % i == 0 && int_len_base_10(num / i) == 3 {
            return Some((i, num / i));
        }
    }

    None
}
