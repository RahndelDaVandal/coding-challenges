// This problem was asked by Facebook.

// Given a string of round, curly, and square open and closing brackets, 
// return whether the brackets are balanced (well-formed).

// For example, given the string "([])[]({})", you should return true.

// Given the string "([)]" or "((()", you should return false.

use std::collections::VecDeque;
use std::fmt;

#[derive(Default)]
pub struct CharStack {
    pub stack: VecDeque<char>,
}
impl CharStack {
    pub fn new() -> Self { Self { stack: VecDeque::new() } }
    pub fn add(&mut self, c: char) { self.stack.push_back(c); }
    pub fn check(&mut self, c: &char) -> bool {
        if self.stack.is_empty() { return false }
        if self.stack.back().unwrap() != c { return false }
        self.stack.pop_back();
        true
    }
}
impl fmt::Display for CharStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut o = String::new();
        o.push('\'');
        let l = self.stack.len() - 1;
        let mut i = 0;
        for c in &self.stack {
            o.push_str(format!("{c},").as_str());
            i += 1;
            if i < l { o.push(' '); } 
        }
        o.push('\'');
        write!(f, "{}", o)
    }
}

pub fn is_balanced(input: &str) -> bool {
    let mut cs = CharStack::new();
    for c in input.chars() {
        match c {
            '[' => { cs.add('['); },
            ']' => { if !cs.check(&'['){ return false } },
            '(' => { cs.add('('); },
            ')' => { if !cs.check(&'('){ return false } },
            '{' => { cs.add('{'); },
            '}' => { if !cs.check(&'{'){ return false } },
            _ => {},
        }
    }
    if !cs.stack.is_empty() { return false }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes() {
        let passes = "([])[]({})";
        let result = is_balanced(passes);
        assert_eq!(result, true);
    }
    #[test]
    fn fails() {
        let fails1 = "([)]";
        let fails2 = "((()";
        let result = is_balanced(fails1);
        assert_eq!(result, false);
        let result = is_balanced(fails2);
        assert_eq!(result, false);
    }
}
