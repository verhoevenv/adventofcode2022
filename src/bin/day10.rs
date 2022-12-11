use std::io;
use std::io::Read;

pub fn register_hist(program: &str) -> Vec<i32> {
    let mut register = 1;
    // the value during a specific cycle
    let mut register_hist =Vec::with_capacity(250);
    // add initial value twice to prevent zero-based indexing messing things up
    register_hist.push(1);
    register_hist.push(1);

    for line in program.lines() {
        let op: Vec<_> = line.split(' ').collect();
        match op[0] {
            "noop" => {
                register_hist.push(register);
            },
            "addx" => {
                let val: i32 = op[1].parse().unwrap();
                register_hist.push(register);
                register += val;
                register_hist.push(register);
            },
            _ => panic!("unknown instruction {}", line)
        }
    }

    return register_hist;
}

pub fn draw(program: &str) -> String {
    let register_hist = register_hist(program);

    let mut screen = String::with_capacity(250);
    let mut beam_pos = -1;
    for cycle in 1..=240 {
        beam_pos += 1;
        if beam_pos == 40 {
            beam_pos = 0;
            screen.push('\n');
        }

        let sprite_pos = register_hist.get(cycle).unwrap();

        if (sprite_pos - beam_pos).abs() <= 1 {
            screen.push('#')
        } else {
            screen.push('.');
        }
    }

    screen.push('\n');

    return screen;
}

pub fn signal_strength(program: &str) -> i32 {
    let register_hist = register_hist(program);

    return [20, 60, 100, 140, 180, 220].iter()
        .map(|c| register_hist.get(*c as usize).unwrap() * c)
        .sum();
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", draw(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_signal_strength() {
        assert_eq!(signal_strength(INPUT), 13140);
    }

    #[test]
    fn test_draw() {
        const EXPECTED: &str = indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
        "};
        assert_eq!(draw(INPUT), EXPECTED);
    }

    const INPUT: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

}