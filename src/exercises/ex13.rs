use std::collections::HashMap;

fn _convert_roman_to_integer_v1(roman: String) -> i32 {
    let mut number = 0;

    for (i, c) in roman.chars().enumerate() {
        number += match c {
            'I' => {
                if let Some(next) = roman.chars().nth(i + 1) {
                    match next {
                        'V' | 'X' => -1,
                        _ => 1,
                    }
                } else {
                    1
                }
            }
            'V' => 5,
            'X' => {
                if let Some(next) = roman.chars().nth(i + 1) {
                    match next {
                        'L' | 'C' => -10,
                        _ => 10,
                    }
                } else {
                    10
                }
            }
            'L' => 50,
            'C' => {
                if let Some(next) = roman.chars().nth(i + 1) {
                    match next {
                        'D' | 'M' => -100,
                        _ => 100,
                    }
                } else {
                    100
                }
            }
            'D' => 500,
            'M' => 1000,
            _ => unreachable!(),
        };
    }
    number
}

fn convert_roman_to_integer(s: String) -> i32 {
    let mut number = 0;
    let roman = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let mut it = s.chars().peekable();
    while let Some(c) = it.next() {
        if let Some(next) = it.peek() {
            if roman.get(&c).unwrap() < roman.get(next).unwrap() {
                number -= roman.get(&c).unwrap();
                continue;
            }
        }

        number += roman.get(&c).unwrap()
    }
    number
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_test1() {
        assert_eq!(convert_roman_to_integer("III".to_owned()), 3)
    }

    #[test]
    fn case_test2() {
        assert_eq!(convert_roman_to_integer("LVIII".to_owned()), 58)
    }

    #[test]
    fn case_test3() {
        assert_eq!(convert_roman_to_integer("MCMXCIV".to_owned()), 1994)
    }
}
