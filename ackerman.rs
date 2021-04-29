fn ack(m: i32, n: i32) -> i32 {
    if (m == 0) {
        return n + 1;
    } else if ((m > 0) && (n == 0)) {
        return ack(m - 1, 1);
    } else if ((m > 0) && (n > 0)) {
        return ack(m - 1, ack(m, n - 1));
    } else {
        panic!("something went wrong");
    }
}

fn main() {
    let a = ack(3, 10);
    println!("{:#?}", a);
}
