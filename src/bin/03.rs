advent_of_code::solution!(3);

fn get_gamma_bits(input: Vec<&str>) -> Vec<bool> {
    let num_bits = match input.get(0) {
        Some(i) => i.trim().len(),
        None => { return vec![] },
    };

    let mut ones: Vec<usize> = vec![0; num_bits];
    
    for binary_string in &input {
        for (index, value) in binary_string.chars().enumerate() {
            // Assume anything not a 1 is a 0
            if value == '1' {
                ones[index] += 1;
            }
        }
    }

    let mut gamma_bits: Vec<bool> = vec![false; num_bits];

    for index in 0..num_bits {
        if ones[index] >= input.len() / 2 {
            gamma_bits[index] = true;
        }
    }

    gamma_bits
}

fn bits_to_num(bits: &Vec<bool>) -> u32 {
    let mut to_return = 0;

    for (index, value) in bits.iter().rev().enumerate() {
        if *value {
            to_return += u32::pow(2, index as u32);
        }
    }

    to_return
}

pub fn part_one(input: &str) -> Option<u32> {
    let gamma_bits = get_gamma_bits(input.lines().collect());

    let gamma = bits_to_num(&gamma_bits);
    let epsilon = bits_to_num(&gamma_bits.iter().map(|b| !b).collect());

    Some(gamma * epsilon)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(198));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
