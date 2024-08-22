fn main() {
    let x: i32 = 0b10110;
    println!("{}", x.leading_zeros());
    println!("{:#b}", x ^ ((1 << 32 - x.leading_zeros()) - 1));
}
