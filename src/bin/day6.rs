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

pub fn four_different(input: &str) -> usize {
    return 4 + 
        input.as_bytes().windows(4).enumerate()
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

    println!("{}", four_different(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_different() {
        assert_eq!(four_different("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(four_different("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(four_different("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(four_different("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(four_different("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

}