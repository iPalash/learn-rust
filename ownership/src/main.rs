fn main() {
    let mut a = [0, 1, 2, 3];
    let (x, rest) = a.split_first_mut().unwrap();
    let y = &rest[0];
    *x *= *y;
}
