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

        else if !self.specials.empty()  && !self.specials.contains('"') && ch != '"'{
            substr.push(ch);
        }
    }
}
