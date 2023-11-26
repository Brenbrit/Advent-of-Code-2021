advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // Convert input to u32 vec
    let mut input = input.lines().map(|s| s.parse::<u32>().expect("Failed to parse u32"));

    let mut num_increases = 0;

    let mut last_depth = match input.next() {
        None => { return Some(0) },
        Some(depth) => depth,
    };

    for depth in input {
        if depth > last_depth {
            num_increases += 1;
        }

        last_depth = depth;
    }

    Some(num_increases)
}

pub fn part_two(input: &str) -> Option<u32> {
    // convert input to u32 vec
    let mut input = input.lines().map(|s| s.parse::<u32>().expect("Failed to parse u32"));

    let mut window: [u32; 3] = [0; 3];

    // Populate first two window values
    for i in 0..3 {
        window[i] = match input.next() {
            None => { return Some(0) },
            Some(j) => j,
        };
    }

    let mut last_window_sum = window[0] + window[1] + window[2];
    let mut increases = 0;

    for next_depth in input {
        // rotate window
        window[0] = window[1];
        window[1] = window[2];
        window[2] = next_depth;

        let window_sum = window[0] + window[1] + window[2];

        if window_sum > last_window_sum {
            increases += 1;
        }

        last_window_sum = window_sum;
    }

    Some(increases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
