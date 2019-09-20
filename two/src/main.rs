fn main() {
    
    let mut s0 = 1;
    let mut s1 = 1;
    let mut sum = 0;

    while sum < 4 * i32::pow(10, 6) {
        let t = s0 + s1;

        if t % 2 == 0 {
            sum += t;
        }
        s0 = s1;
        s1 = t;
    }

    println!("{}", sum);
}
