use std::collections::HashMap;

pub struct Splitter {
    haystack: String,
    delimiter: char,
    ignore_special: bool,
}

impl Splitter {
    pub fn new(haystack: String, delimiter: char, ignore_special: bool) -> Self {
        Self {
            haystack,
            delimiter,
            ignore_special,
        }
    }

    pub fn set_delimiter(&mut self, delimiter: char) {
        self.delimiter = delimiter;
    }

    pub fn set_haystack(&mut self, haystack: String) {
        self.haystack = haystack;
    }

    pub fn ignore_special_characters(&mut self) {
         self.ignore_special = true;
    }

    pub fn allow_special_characters(&mut self) {
        self.ignore_special = false;
    }
    
    pub fn get_delimiter(&self) -> char {
       self.delimiter 
    }

    pub fn get_haystack(&self) -> String {
        self.haystack.clone()
    }

    pub fn split(&self) -> Vec<String> {
        if self.ignore_special {
            return self.split_ignore();
        }
        else{
            return self.split_attention();
        }
    }

    fn split_ignore(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::default();
        let mut _string = String::default();
        for ch in self.haystack.chars() {
           if ch == self.delimiter {
                if !_string.is_empty() {
                    result.push(_string);
                    _string = String::default();
                }
            } 
            else{
                _string += &format!("{}",ch).to_string();
            }
        }
        return result;
    }

    fn split_attention(&self) -> Vec<String> {
        todo!();
    }
}


#[cfg(test)]
mod test{

    use super::Splitter;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_splitter_new() {
        let _test_literal = Splitter::new("Hello".to_string(), 'W', true);
        let mut _string = String::default();
        let mut rng = thread_rng();
        // not known at compile time
        for _i in 0..100 {
            _string += &(rng.gen::<i32>() % 120).to_string();
        }
        let _test_string = Splitter::new(_string, 'W', true);
    }
}



