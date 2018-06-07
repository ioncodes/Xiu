pub struct Stack {
    buffer: Vec<u16>
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            buffer: Vec::<u16>::new()
        }
    }

    pub fn push(&mut self, data: u16) {
        self.buffer.push(data);
    }

    pub fn pop(&mut self) -> u16 {
        if let Some(x) = self.buffer.pop() {
            x
        } else {
            panic!("Can't pop from empty stack!");
        }
    }
}
