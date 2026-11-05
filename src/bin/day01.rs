fn main() {
    let input = include_str!("../../input/day01.txt");

    let mut dial = 50;
    let mut count = 0;

    for line in input.lines() {
        let (direction, number) = line.split_at(1);
        let amount: i32 = number.parse().unwrap();

        let clicks = match direction {
            "L" => -amount,
            "R" => amount,
            _ => panic!("unknown direction {direction}"),
        };

        dial = (dial + clicks).rem_euclid(100); //or ((dial + clicks) % 100 + 100) % 100;

        if dial == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}
