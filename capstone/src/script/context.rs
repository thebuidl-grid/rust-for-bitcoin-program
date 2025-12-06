use std::collections::VecDeque;

/// Script execution context with stack and state
#[derive(Debug, Clone)]
pub struct ScriptContext {
    /// Execution stack
    pub stack: VecDeque<Vec<u8>>,
    /// Execution log for debugging
    pub log: Vec<String>,
    /// Whether execution should continue
    pub should_continue: bool,
    /// SIGHASH for signature verification (optional)
    pub sighash: Option<Vec<u8>>,
}

impl ScriptContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            log: Vec::new(),
            should_continue: true,
            sighash: None,
        }
    }

    /// Create context with SIGHASH for signature verification
    pub fn with_sighash(sighash: Vec<u8>) -> Self {
        Self {
            stack: VecDeque::new(),
            log: Vec::new(),
            should_continue: true,
            sighash: Some(sighash),
        }
    }

    /// Log a message
    pub fn log(&mut self, message: String) {
        self.log.push(message.clone());
        if cfg!(feature = "verbose") {
            println!("  {}", message);
        }
    }

    /// Push data onto stack
    pub fn push(&mut self, data: Vec<u8>) {
        self.stack.push_back(data.clone());
        self.log(format!("PUSH: {} bytes", data.len()));
    }

    /// Pop data from stack
    pub fn pop(&mut self) -> Option<Vec<u8>> {
        let result = self.stack.pop_back();
        if result.is_some() {
            self.log(format!("POP: {} bytes", result.as_ref().unwrap().len()));
        } else {
            self.log("POP: stack empty!".to_string());
        }
        result
    }

    /// Peek at stack item (0 = top)
    pub fn peek(&self, index: usize) -> Option<&Vec<u8>> {
        if index < self.stack.len() {
            Some(&self.stack[self.stack.len() - 1 - index])
        } else {
            None
        }
    }

    /// Check if script execution was successful
    pub fn is_valid(&self) -> bool {
        match self.stack.back() {
            Some(top) => !top.is_empty() && !(top.len() == 1 && top[0] == 0),
            None => false,
        }
    }
}

impl Default for ScriptContext {
    fn default() -> Self {
        Self::new()
    }
}

