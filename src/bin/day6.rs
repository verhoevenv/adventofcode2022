use std::io;
use std::io::Read;

pub fn all_different<T: PartialEq>(input: &[T]) -> bool {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[i] == input[j] {
                return false;
            }
        }
    }
    return true;
}

pub fn different(input: &str, len: usize) -> usize {
    return len + 
        input.as_bytes().windows(len).enumerate()
        .filter(|(_, s)| all_different(s))
        .map(|(i, _)| i)
        .nth(0)
        .unwrap();
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");

    println!("{}", different(&input, 14));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_different() {
        assert_eq!(different("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(different("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(different("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(different("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(different("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_fourteen_different() {
        assert_eq!(different("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(different("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(different("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(different("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(different("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

}