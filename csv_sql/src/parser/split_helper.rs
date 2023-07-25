pub fn get_bounds(substr: &String) -> Option<(usize, usize)> {
    if substr.len() == 0 {
        return None;
    }

    let mut hit_begin = false;
    let mut hit_end = false;
    let mut left = 0;
    let mut right = substr.len() - 1;

    while left < right && !(hit_begin && hit_end) {
        if !check_nth_whitespace(&substr, left) && !hit_begin {
            hit_begin = true;
        }
        if !check_nth_whitespace(&substr, right) && !hit_end {
            hit_end = true;
        }
        left += !hit_begin as usize;
        right -= !hit_end as usize;
    }

    if left >= right {
        return None;
    }

    return Some((left, right));
}

fn get_nth(substr: &String, index: usize) -> Option<char> {
    return substr.chars().nth(index);
}

fn check_nth_whitespace(substr: &String, index: usize) -> bool {
    match get_nth(&substr, index) {

        Some(ch) => ch.is_whitespace(),

        None => false,
    }
}

pub fn get_slice(substr: &String, bounds: Option<(usize, usize)>) -> Option<String> {
    match bounds {
        Some(borders) => {

            let mut res = String::default();

            for i in borders.0 ..= borders.1 {
                match get_nth(&substr, i) {

                    Some(ch) => res.push(ch),

                    None => {return None;},
                }
            }

            Some(res)
        },
        None => None,
    }
}

#[test]
fn test_get_bounds_empty_none() {
    match get_bounds(&String::default()) {
        Some(_fail_me) => {assert!(false)},
        None => {},
    }
}

#[test]
fn test_get_bounds_whitespace_none() {
    match get_bounds(&"        \n   \t              ".to_string()) {
        Some(_fail_me) => {assert!(false)},
        None => assert!(true),
    }
}

#[test]
fn test_get_bounds_some() {
    match get_bounds(&"   this   ".to_string()) {
        Some(bounds) => {
            assert_eq!(bounds.0, 3);
            assert_eq!(bounds.1, 6);
        },
        None => assert!(false),
    }
    match get_bounds(&"012345".to_string()){
        Some(bounds) => {
            assert_eq!(bounds.0, 0);
            assert_eq!(bounds.1, 5);
        },
        None => assert!(false),
    }
}

#[test]

fn test_get_slice() {
    let target = "     \t \n\n\n        testing          \n\n\n\t".to_string();
    match get_slice(&target, get_bounds(&target)){
        Some(substr) => assert_eq!(substr, "testing".to_string()),
        None => assert!(false),
    }
}
