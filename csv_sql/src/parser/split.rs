use super::stack::Stack;
use super::split_helper::{get_slice, get_bounds};

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
            match get_bounds(&substr) {
                Ok(boundary) => {
                    match get_slice(&substr, boundary) {
                    Ok(new_string) => new_body.push(new_string),
                    Err(msg) => panic!("{}", msg),
                    }

                },
                Err(_msg) => {/*empty or all whitespace discarded*/},
            };
        }

        self.body = new_body;
    }
}
