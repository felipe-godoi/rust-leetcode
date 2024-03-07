use std::collections::HashMap;

fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    let mut valid = true;
    let openers = HashMap::from([(')', '('), (']', '['), ('}', '{')]);

    for c in s.chars() {
        match c {
            ')' | ']' | '}' => {
                if let Some(last) = stack.pop() {
                    if last == *openers.get(&c).unwrap() {
                        continue;
                    }
                }
                valid = false;
                break;
            }
            _ => stack.push(c),
        }
    }

    valid && stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn test_case2() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_case3() {
        assert_eq!(is_valid("(]".to_string()), false);
    }
}
