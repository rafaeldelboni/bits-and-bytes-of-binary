type Bits = Vec<u8>;

pub fn bits_to_number (bits: Bits) -> u32 {
    bits.iter()
        .enumerate()
        .fold(0u32, |acc, (iter, bit)| {
            let power_of_two = bits.len() - iter - 1;
            if bit.clone() == 1 {
                acc + 2u32.pow(power_of_two as u32)
            } else {
                acc
            }
        })
}

pub fn number_to_bits (number: u32) -> Bits {
    let quot = number / 2u32;
    let rem = number % 2u32;
    if quot == 0 {
        vec![rem as u8]
    } else {
        [number_to_bits(quot), vec![rem as u8]].concat()
    }
}

fn assert_same_number_of_bits (left: Bits, right: Bits) -> bool {
    left.len() == right.len()
}

fn apply_both_bits <F>(left: Bits, right: Bits, mut apply: F) -> Bits
    where F: FnMut(u8, u8) -> u8
{
    if !assert_same_number_of_bits(left.clone(), right.clone()) {
        panic!("Bit mismatch (l= {}, r= {})", left.len(), right.len());
    }

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| apply(*a, *b))
        .collect()
}

pub fn and (left: Bits, right: Bits) -> Bits {
    apply_both_bits(left, right, std::ops::BitAnd::bitand)
}

pub fn or (left: Bits, right: Bits) -> Bits {
    apply_both_bits(left, right, std::ops::BitOr::bitor)
}

pub fn xor (left: Bits, right: Bits) -> Bits {
    apply_both_bits(left, right, std::ops::BitXor::bitxor)
}

pub fn not (bits: Bits) -> Bits {
    bits.iter()
        .map(|bit| if *bit == 1u8 {0} else {1})
        .collect()
}

fn size_diff_bits (left: &Bits, right: &Bits) -> usize {
    (left.len() as isize - right.len() as isize).max(0) as usize
}

fn zero_extend_size(bits: &Bits, size: usize) -> Bits {
    [vec![0; size], bits.clone()].concat()
}

pub fn add (left: Bits, right: Bits) -> Bits {
    let zero_fill_left = zero_extend_size(&left, size_diff_bits(&right, &left));
    let zero_fill_right = zero_extend_size(&right, size_diff_bits(&left, &right));

    let sum_bits_tuple = zero_fill_left.iter().rev()
        .zip(zero_fill_right.iter().rev())
        .fold((0, Bits::new()), |(carry_bit, sum_bits), (left, right)|
            {
                let lr_sum = left ^ right;
                let lr_carry = left & right;

                let lr_sum_plus_carry = lr_sum ^ carry_bit;
                let lr_sum_carry = lr_sum & carry_bit;

                (
                    lr_carry | lr_sum_carry,
                    [sum_bits, vec![lr_sum_plus_carry]].concat(),
                )
            }
        );

    [vec![sum_bits_tuple.0], sum_bits_tuple.1.into_iter().rev().collect()].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_number() {
        assert_eq!(bits_to_number(vec![0, 0, 0, 0]), 0);
        assert_eq!(bits_to_number(vec![0, 0, 0, 1]), 1);
        assert_eq!(bits_to_number(vec![0, 0, 1, 0]), 2);
        assert_eq!(bits_to_number(vec![0, 0, 1, 1]), 3);
        assert_eq!(bits_to_number(vec![0, 1, 0, 0]), 4);
        assert_eq!(bits_to_number(vec![0, 1, 0, 1]), 5);
        assert_eq!(bits_to_number(vec![0, 1, 1, 0]), 6);
        assert_eq!(bits_to_number(vec![0, 1, 1, 1]), 7);
        assert_eq!(bits_to_number(vec![1, 0, 0, 0]), 8);
        assert_eq!(bits_to_number(vec![1, 0, 0, 1]), 9);
        assert_eq!(bits_to_number(vec![1, 0, 1, 0]), 10);
        assert_eq!(bits_to_number(vec![1, 0, 1, 1]), 11);
        assert_eq!(bits_to_number(vec![1, 1, 0, 0]), 12);
        assert_eq!(bits_to_number(vec![1, 1, 0, 1]), 13);
        assert_eq!(bits_to_number(vec![1, 1, 1, 0]), 14);
        assert_eq!(bits_to_number(vec![1, 1, 1, 1]), 15);
    }

    #[test]
    fn test_number_to_bits() {
        assert_eq!(number_to_bits(0), vec![0]);
        assert_eq!(number_to_bits(1), vec![1]);
        assert_eq!(number_to_bits(2), vec![1, 0]);
        assert_eq!(number_to_bits(3), vec![1, 1]);
        assert_eq!(number_to_bits(4), vec![1, 0, 0]);
        assert_eq!(number_to_bits(5), vec![1, 0, 1]);
        assert_eq!(number_to_bits(6), vec![1, 1, 0]);
        assert_eq!(number_to_bits(7), vec![1, 1, 1]);
        assert_eq!(number_to_bits(8), vec![1, 0, 0, 0]);
        assert_eq!(number_to_bits(9), vec![1, 0, 0, 1]);
        assert_eq!(number_to_bits(10), vec![1, 0, 1, 0]);
        assert_eq!(number_to_bits(11), vec![1, 0, 1, 1]);
        assert_eq!(number_to_bits(12), vec![1, 1, 0, 0]);
        assert_eq!(number_to_bits(13), vec![1, 1, 0, 1]);
        assert_eq!(number_to_bits(14), vec![1, 1, 1, 0]);
        assert_eq!(number_to_bits(15), vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_assert_same_number_of_bits() {
        assert_eq!(assert_same_number_of_bits(vec![0], vec![0]), true);
        assert_eq!(assert_same_number_of_bits(vec![0, 1], vec![0]), false);
    }

    #[test]
    #[should_panic(expected = "Bit mismatch (l= 2, r= 1)")]
    fn test_apply_both_bits_panic() {
        apply_both_bits(vec![1, 1], vec![1], std::ops::Add::add);
    }

    #[test]
    fn test_and() {
        assert_eq!(and(vec![1, 0, 1, 1], vec![1, 0, 0, 1]), vec![1, 0, 0, 1]);
        assert_eq!(and(vec![1, 1, 1, 1], vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
        assert_eq!(and(vec![0, 0, 0, 0], vec![1, 1, 1, 1]), vec![0, 0, 0, 0]);
        assert_eq!(and(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), vec![1, 1, 1, 1]);
        assert_eq!(and(vec![0, 0, 0, 0], vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(vec![1, 0, 1, 1], vec![1, 0, 0, 1]), vec![1, 0, 1, 1]);
        assert_eq!(or(vec![1, 1, 1, 1], vec![0, 0, 0, 0]), vec![1, 1, 1, 1]);
        assert_eq!(or(vec![0, 0, 0, 0], vec![1, 1, 1, 1]), vec![1, 1, 1, 1]);
        assert_eq!(or(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), vec![1, 1, 1, 1]);
        assert_eq!(or(vec![0, 0, 0, 0], vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(vec![1, 0, 1, 1], vec![1, 0, 0, 1]), vec![0, 0, 1, 0]);
        assert_eq!(xor(vec![1, 1, 1, 1], vec![0, 0, 0, 0]), vec![1, 1, 1, 1]);
        assert_eq!(xor(vec![0, 0, 0, 0], vec![1, 1, 1, 1]), vec![1, 1, 1, 1]);
        assert_eq!(xor(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), vec![0, 0, 0, 0]);
        assert_eq!(xor(vec![0, 0, 0, 0], vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_not() {
        assert_eq!(not(vec![1, 0, 1, 1]), vec![0, 1, 0, 0]);
        assert_eq!(not(vec![1, 1, 1, 1]), vec![0, 0, 0, 0]);
        assert_eq!(not(vec![0, 0, 0, 0]), vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_zero_extend_size() {
        assert_eq!(zero_extend_size(&vec![1], 4), vec![0, 0, 0, 0, 1]);
        assert_eq!(zero_extend_size(&vec![1, 1], 4), vec![0, 0, 0, 0, 1, 1]);
        assert_eq!(zero_extend_size(&vec![1, 0, 1], 4), vec![0, 0, 0, 0, 1, 0, 1]);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(vec![1], vec![1]), vec![1, 0]);
        assert_eq!(add(vec![1, 1, 1], vec![1, 0, 1, 0, 0]), vec![0, 1, 1, 0, 1, 1]);
        assert_eq!(add(vec![1, 0, 0], vec![1, 0, 0, 0, 0]), vec![0, 1, 0, 1, 0, 0]);
    }
}
