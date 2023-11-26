advent_of_code::solution!(2);

enum Instruction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl From<&str> for Instruction {
    // Will panic if translation is not possible
    fn from(item: &str) -> Self {
        let parts: Vec<&str> =  item.trim().split(' ').collect();

        // Instructions should have only one space
        if parts.len() < 2 {
            panic!("Instruction should contain at least 2 terms.");
        }

        let instruction_type = *parts.get(0).unwrap();
        let distance = parts.get(1).unwrap().parse::<u32>().expect("Failed to parse instruction distance.");

        if instruction_type.eq("forward") {
            Instruction::Forward(distance)
        } else if instruction_type.eq("up") {
            Instruction::Up(distance)
        } else if instruction_type.eq("down") {
            Instruction::Down(distance)
        } else {
            panic!("Unknown instruction.");
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // Convert input to Instruction vec
    let input = input.lines().map(|s| Instruction::from(s));

    let mut depth = 0;
    let mut distance = 0;

    for instruction in input {
        match instruction {
            Instruction::Forward(i) => { distance += i },
            Instruction::Up(i) => {
                if i > depth {
                    depth = 0;
                    println!("Negative depth");
                } else {
                    depth -= i;
                }
            },
            Instruction::Down(i) => { depth += i },
        }
    }

    Some(depth * distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Convert input to Instruction vec
    let input = input.lines().map(|s| Instruction::from(s));

    let mut depth = 0;
    let mut distance = 0;
    let mut aim: u32 = 0; // we assume aim can never be negative

    for instruction in input {
        match instruction {
            Instruction::Forward(i) => {
                distance += i;
                depth += aim * i;
            },
            Instruction::Up(i) => {
                if i > aim {
                    aim = 0;
                } else {
                    aim -= i;
                }
            },
            Instruction::Down(i) => { aim += i },
        }
    }

    Some(depth * distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(900));
    }
}
