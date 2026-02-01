use crate::error::VMError;
use crate::gas::GasMeter;
use crate::instruction::Instruction;
use crate::stack::Stack;
use latte_primitives::address::Address;
use latte_state::account_db::AccountWriter;

pub struct Interpreter<'a, S: AccountWriter> {
    pub state: &'a mut S,
    pub caller: Address,
    pub pc: usize,
    pub stack: Stack,
    pub gas: GasMeter,
}

///
/// 对pc的检查，放置越界
/// 除0的异常
/// 相似的重复代码抽取为宏
impl<'a, S: AccountWriter> Interpreter<'a, S> {
    pub fn execute(&mut self, code: &[Instruction]) -> Result<(), VMError> {
        while self.pc < code.len() {
            // 获取当前指令并提前增加 PC，Jump 指令会覆盖它
            let instruction = &code[self.pc];
            self.pc += 1;

            // 扣费
            self.gas.charge(1)?;
            match instruction {
                Instruction::Push(v) => {
                    self.stack.push(*v);
                }
                Instruction::Add => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a + b);
                }
                Instruction::Sub => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a - b);
                }
                Instruction::Mul => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a * b);
                }
                Instruction::Div => {
                    let b = self.stack.pop()?;
                    if b == 0 {
                        return Err(VMError::DivideByZero);
                    }
                    let a = self.stack.pop()?;
                    self.stack.push(a / b);
                }
                Instruction::Eq => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push((a == b) as i64);
                }
                Instruction::Gt => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push((a > b) as i64);
                }
                Instruction::Lt => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push((a < b) as i64);
                }
                Instruction::Load => {
                    let key = self.stack.pop()? as u64;
                    if let Some(account) = self.state.get_mut(&self.caller) {
                        // 从站定取出账号的key，然后根据caller从state获取账户，从account中根据key获取值，如果获取不成功，则取默认值0
                        let value = account
                            .storage
                            .get(&key.to_be_bytes().to_vec())
                            .map(|v| i64::from_be_bytes(v.clone().try_into().unwrap()))
                            .unwrap_or(0);
                        // 和其他指令一样，将值写入栈
                        self.stack.push(value);
                    }
                }
                Instruction::Store => {
                    let key = self.stack.pop()? as u64;
                    let val = self.stack.pop()?;
                    if let Some(account) = self.state.get_mut(&self.caller) {
                        account
                            .storage
                            .insert(key.to_be_bytes().to_vec(), val.to_be_bytes().to_vec());
                    }
                }
                Instruction::Jump(target) => {
                    if *target >= code.len() {
                        return Err(VMError::InvalidJump);
                    }
                    self.pc = *target;
                    continue;
                }
                Instruction::JumpIf(target) => {
                    let cond = self.stack.pop()?;
                    if cond != 0 {
                        self.pc = *target;
                        continue;
                    }
                }
                Instruction::Dup => {
                    // 复制栈顶元素
                    self.stack.dup()?;
                }
                Instruction::Pop => {
                    // 仅仅弹出栈顶元素并丢弃
                    self.stack.pop()?;
                }
                Instruction::Return => {
                    break;
                }
            }
        }

        Ok(())
    }
}
