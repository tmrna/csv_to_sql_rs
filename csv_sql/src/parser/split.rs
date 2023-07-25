use super::stack::Stack;

pub struct Split {
    haystack: String,
    delimiter: char,
    specials: Stack<char>,
    body: Vec<String>,
}

impl Split {
    pub fn new(haystack: String, delimiter: char) -> Self {
        Split {
            haystack,
            delimiter,
            specials: Stack::default(),
            body: Vec::default(),
        }
    }

    pub fn split(&mut self) -> Vec<String> {
        if !self.body.is_empty() {
            return self.body.clone();
        }
        
        self.load_splitter();
        self.body.clone()
    }

    fn load_splitter(&mut self) {
        let mut substr = String::default();
        let haystack = self.haystack.clone();
        for ch in haystack.chars() {
            if ch.is_ascii_punctuation() || !self.specials.empty() {
                self.handle_punctuation(ch, &mut substr);
            }
            else if ch == self.delimiter && !self.specials.contains('"') {
                //self.load_substr(&mut substr);
            }
            else {
                substr.push(ch);
            }
        }

        self.strip_substr_whitespace();
    }

    fn handle_punctuation(&mut self, ch: char, substr:&mut String) {
        if self.specials.empty(){
            match ch {
                '[' | '{' | '\'' | '(' => {
                    substr.push(ch);
                    self.specials.push(ch);
                },
                '"' => {
                    self.specials.push(ch);
                }
                _ => {
                    substr.push(ch);
                },
            }
        }

        else if self.specials.contains('"') && ch != '"' {
            substr.push(ch);
        }

        else if !self.specials.contains('"') && ch == '"' {

        } 
    }

    fn strip_substr_whitespace(&mut self) {
        let mut new_body = Vec::default();
        for substr in self.body.clone() {
            match get_slice(&substr, get_bounds(&substr)) {
                Some(new_string) => new_body.push(new_string),
                None => {},
            }
        }
        self.body = new_body;
    }
}

////////////////////////////  HELPER //////////////////////////////////

fn get_bounds(substr: &String) -> Option<(usize, usize)> {
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

fn get_slice(substr: &String, bounds: Option<(usize, usize)>) -> Option<String> {
    match bounds {
        Some(borders) => {

            let mut res = String::default();

            for i in borders.0 .. borders.1 {
                match get_nth(&substr, i) {

                    Some(ch) => res.push(ch),

                    None => {return None;},
                }
            }

            return Some(res);
        },
        None => None,
    }
}
