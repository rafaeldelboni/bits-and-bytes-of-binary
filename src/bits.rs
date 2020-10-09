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

fn assert_same_number_of_bits (bits_a: Bits, bits_b: Bits) -> bool {
    bits_a.len() == bits_b.len()
}

pub fn and (bits_a: Bits, bits_b: Bits) -> Bits {
    if !assert_same_number_of_bits(bits_a.clone(), bits_b.clone()) {
        panic!("Bit mismatch (a= {}, b = {})", bits_a.len(), bits_b.len());
    }

    bits_a.iter()
        .zip(bits_b.iter())
        .map(|(a, b)| a & b)
        .collect()
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
    fn test_and() {
        assert_eq!(and(vec![1, 0, 1, 1], vec![1, 1, 0, 1]), vec![1, 0, 0, 1]);
        assert_eq!(and(vec![1, 1, 1, 1], vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);
        assert_eq!(and(vec![0, 0, 0, 0], vec![1, 1, 1, 1]), vec![0, 0, 0, 0]);
        assert_eq!(and(vec![1, 1, 1, 1], vec![1, 1, 1, 1]), vec![1, 1, 1, 1]);
    }

    #[test]
    #[should_panic(expected = "Bit mismatch (a= 2, b = 1)")]
    fn test_and_panic() {
        and(vec![1, 1], vec![1]);
    }
}
