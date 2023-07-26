use std::result::Result;

pub fn get_bounds(substr: &String) -> Result<(usize, usize), String> {
    if substr.len() == 0 {
        return Err("Empty string".to_string());
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
        return Err("String is all whitespace".to_string());
    }

    return Ok((left, right));
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

pub fn get_slice(substr: &String, bounds: (usize, usize)) -> Result<String, String> {
    let mut res = String::default();
    for index in bounds.0 ..= bounds.1 {
        match get_nth(substr, index) {
            Some(ch) => res.push(ch),
            None => {return Err("String length exceeded".to_string());}
        }
    }
    Ok(res)
}

#[test]
fn test_get_bounds_empty_none() {
    match get_bounds(&String::default()) {
        Ok(_fail_me) => {assert!(false)},
        Err(msg) => {},
    }
}

#[test]
fn test_get_bounds_whitespace_none() {
    match get_bounds(&"        \n   \t              ".to_string()) {
        Ok(_fail_me) => {assert!(false)},
        Err(_msg) => assert!(true),
    }
}

#[test]
fn test_get_bounds_some() {
    match get_bounds(&"   this   ".to_string()) {
        Ok(bounds) => {
            assert_eq!(bounds.0, 3);
            assert_eq!(bounds.1, 6);
        },
        Err(_msg) => assert!(false),
    }
    match get_bounds(&"012345".to_string()){
        Ok(bounds) => {
            assert_eq!(bounds.0, 0);
            assert_eq!(bounds.1, 5);
        },
        Err(_msg) => assert!(false),
    }
}

#[test]
fn test_get_slice() {
    let target = "     \t \n\n\n        testing          \n\n\n\t".to_string();
    let bounds = match get_bounds(&target) {
        Ok(boundary) => boundary,
        Err(msg) => panic!("could not properly obtain boundary, err msg: {}", msg),
    };
    match get_slice(&target, bounds){
        Ok(substr) => assert_eq!(substr, "testing".to_string()),
        Err(_msg) => assert!(false),
    }
}
