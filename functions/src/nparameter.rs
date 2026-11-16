fn looper() {
    let mut count: i32 = 0;
    loop {
        if count == 10 {
            break;
        } else {
            println!("Count is: {}", count);
        }
        count += 1;
    }
}

fn main() {
    looper();
}