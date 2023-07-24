#[derive(Default)]
pub struct Stack<T: Clone + PartialEq> {
    contents: Vec<T>,
}

impl<T: Clone + PartialEq> Stack<T> {

    pub fn tos(&self) -> Option<T> {
        match self.contents.last() {
            Some(res) => Some(res.clone()),
            None => None,
        }
    }

    pub fn push(&mut self, payload: T) {
        self.contents.push(payload);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.contents.pop()
    }

    pub fn size(&self) -> usize {
        self.contents.len()
    }

    pub fn empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn clear(&mut self) {
        self.contents.clear();
    }

    pub fn upper_index(&self) -> Option<usize> {
        match self.size() {
            0 => None,
            _ => Some(self.size() - 1),
        }
    }

    pub fn contains(&self, target: T) -> bool {
        let mut right = match self.upper_index() {
            Some(index) => index,
            None => return false,
        };
        let mut left = 0;

        while left < right {
            if self.get(left).unwrap() == target || self.get(right).unwrap() == target {
                return true;
            }
            left += 1;
            right -= 1;
        }
        return false;
    }

    fn get(&self, index: usize) -> Option<T> {
        match self.upper_index() {
            Some(_res) => Some(self.contents[index].clone()),
            None => None,
        }
    }
}

#[test]
fn test_tos() {
    let mut stk: Stack<String> = Stack::default();
    stk.push("Hello".to_string());
    assert_eq!(stk.tos().unwrap(), "Hello".to_string());
}

#[test]
fn test_contains() {
    let mut stk: Stack<usize> = Stack::default();
    stk.push(4);
    stk.push(8);
    stk.push(16);
    stk.push(32);
    assert!(stk.contains(8));
    assert!(stk.contains(4));
    assert!(stk.contains(16));
    assert!(stk.contains(32));
}
