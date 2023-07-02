fn main() {
    // Another way to get carry without bit shifts?
    // more readable? 
    let x: u32 = 65600;
    let leading_zeros = x.leading_zeros();
    let bit_length = 32 - leading_zeros;
    let carry = 32 - x.leading_zeros() > 16;

    println!("number {:#04x} {}", x, x);
    println!("Bit length of the number: {}", bit_length);
    println!("Carry: {}", carry);
}