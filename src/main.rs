mod bits;

fn main() {
    println!("Decimal: [1, 1, 0, 0]");
    println!("{}", bits::bits_to_number(vec![1, 1, 0, 0]));

    println!("Binary: 17");
    println!("{:?}", bits::number_to_bits(174u32));

    println!("AND: [1, 0, 1, 1] & [1, 0, 0, 1]");
    println!("{:?}", bits::and(vec![1, 0, 1, 1], vec![1, 0, 0, 1]));

    println!("OR: [1, 0, 1, 1] & [1, 0, 0, 1]");
    println!("{:?}", bits::or(vec![1, 0, 1, 1], vec![1, 0, 0, 1]));

    println!("XOR: [1, 0, 1, 1] & [1, 0, 0, 1]");
    println!("{:?}", bits::xor(vec![1, 0, 1, 1], vec![1, 0, 0, 1]));

    println!("NOT: [1, 0, 1, 1]");
    println!("{:?}", bits::not(vec![1, 0, 1, 1]));
}
