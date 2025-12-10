use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct Stack {
    items: Vec<Vec<u8>>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: Vec<u8>) {
        self.items.push(item.clone());
        println!("  Stack PUSH: {} bytes", item.len());
        self.print_state();
    }

    pub fn pop(&mut self) -> Result<Vec<u8>> {
        self.items
            .pop()
            .ok_or_else(|| anyhow!("Stack underflow: attempted to pop from empty stack"))
    }

    pub fn peek(&self) -> Result<&Vec<u8>> {
        self.items
            .last()
            .ok_or_else(|| anyhow!("Stack is empty: cannot peek"))
    }

    pub fn dup(&mut self) -> Result<()> {
        let top = self.peek()?.clone();
        self.push(top);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get(&self, index: usize) -> Result<&Vec<u8>> {
        self.items
            .get(index)
            .ok_or_else(|| anyhow!("Stack index out of bounds: {}", index))
    }

    pub fn print_state(&self) {
        println!("  Stack (top to bottom):");
        if self.items.is_empty() {
            println!("    [empty]");
        } else {
            for (i, item) in self.items.iter().rev().enumerate() {
                println!("    [{}]: {}", i, hex::encode(item));
            }
        }
    }

    pub fn top(&self) -> Result<bool> {
        let top = self.peek()?;
        Ok(!top.is_empty() && top != &vec![0])
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        stack.push(vec![1, 2, 3]);
        assert_eq!(stack.len(), 1);
        let item = stack.pop().unwrap();
        assert_eq!(item, vec![1, 2, 3]);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_pop_empty_fails() {
        let mut stack = Stack::new();
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_dup() {
        let mut stack = Stack::new();
        stack.push(vec![1, 2, 3]);
        stack.dup().unwrap();
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop().unwrap(), vec![1, 2, 3]);
        assert_eq!(stack.pop().unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        stack.push(vec![1, 2, 3]);
        assert_eq!(stack.peek().unwrap(), &vec![1, 2, 3]);
        assert_eq!(stack.len(), 1);
    }
}
