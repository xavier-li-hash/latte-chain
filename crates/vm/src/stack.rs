use crate::error::VMError;

#[derive(Default)]
pub struct Stack {
    inner: Vec<i64>,
}

impl Stack {
    pub fn push(&mut self, value: i64) {
        self.inner.push(value);
    }

    pub fn pop(&mut self) -> Result<i64, VMError> {
        self.inner.pop().ok_or(VMError::StackUnderflow)
    }

    pub fn dup(&mut self) -> Result<(), VMError> {
        let value = *self.inner.last().ok_or(VMError::StackUnderflow)?;
        self.inner.push(value);
        Ok(())
    }
}
