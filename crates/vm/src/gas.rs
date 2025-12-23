use crate::error::VMError;

#[derive(Debug)]
pub struct GasMeter {
    remaining: u64,
}

impl GasMeter {
    pub fn new(remaining: u64) -> Self {
        Self { remaining }
    }

    pub fn charge(&mut self, amount: u64) -> Result<(), VMError> {
        if self.remaining < amount {
            return Err(VMError::OutOfGas);
        }
        self.remaining -= amount;
        Ok(())
    }
}
