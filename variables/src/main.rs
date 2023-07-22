use std::io;

// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Not a valid index");

    let index = index.trim().parse::<usize>().expect("Not a number");

    println!(
        "The val of chosen element at index {} is {}",
        index, a[index],
    );
}
