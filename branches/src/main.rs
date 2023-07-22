fn main() {
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was somthing nonzero");
    }

    if number % 4 == 0 {
        println!("number divisible by 4");
    } else if number % 3 == 0 {
        println!("number divisible by 3");
    } else if number % 2 == 0 {
        println!("number divisible by 2");
    } else {
        println!("number not divisible by 2,3,4");
    }

    let cond = true;
    let number = if cond { 5 } else { 6 };

    println!("value of number is {number}");
}
