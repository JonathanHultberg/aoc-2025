fn main() {
    let input = include_str!("../../input/day01.txt");
    let (part1, part2) = solve(input);

    println!("Part 1: {part1}");
    println!("---------------");
    println!("Part 2: {part2}");
}

fn solve(input: &str) -> (i32, i32) {
    let mut pos = 50;
    let mut stops_at_zero = 0;
    let mut total_zero_hits = 0;

    for line in input.lines() {
        let (dir, count_str) = line.split_at(1);
        let steps: i32 = count_str.parse().unwrap();

        let (dist_to_zero, delta) = match dir {
            "R" => ((100 - pos) % 100, steps),
            "L" => (pos, -steps),
            _ => panic!("unknown direction {dir}"),
        };

        let dist_to_zero = if dist_to_zero == 0 { 100 } else { dist_to_zero };

        if steps >= dist_to_zero {
            let rem_steps = steps - dist_to_zero;
            total_zero_hits += 1 + (rem_steps / 100);
        }

        pos = (pos + delta).rem_euclid(100);

        if pos == 0 {
            stops_at_zero += 1;
        }
    }

    (stops_at_zero, total_zero_hits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../../input/day01_example.txt");
        assert_eq!(solve(input), (3, 6));
    }

    #[test]
    fn r1000() {
        assert_eq!(solve("R1000"), (0, 10));
    }
}
