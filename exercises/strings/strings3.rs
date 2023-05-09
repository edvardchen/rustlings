// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut begin = 0;
    let l = input.len();
    let mut end = l - 1;
    for char in input.chars() {
        if char == ' ' {
            begin += 1
        } else {
            break;
        }
    }
    if begin == l {
        return String::new();
    }
    for char in input.chars().rev() {
        if char == ' ' {
            end -= 1
        } else {
            break;
        }
    }
    input[begin..=end].to_string()
}

fn compose_me(input: &str) -> String {
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let to_match: Vec<char> = "cars".chars().collect();
    let l = to_match.len();
    let mut next_match_index = 0;
    let mut end_index = None;
    for (i, char) in input.char_indices() {
        if char == to_match[next_match_index] {
            next_match_index += 1;
            // reach the end
            if next_match_index == l {
                end_index = Some(i);
                break;
            }
        } else {
            // reset
            next_match_index = 0
        }
    }
    if let Some(end) = end_index {
        format!("{}balloons{}", &input[0..=end - l], &input[end + 1..])
    } else {
        input.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
