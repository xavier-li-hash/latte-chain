use crate::gas::GasMeter;
use crate::instruction::Instruction;
use crate::interpreter::Interpreter;
use latte_primitives::address::Address;
use latte_state::error::StateError;
use latte_state::state::WorldState;
use latte_state::vm::VmEngine;
use latte_types::transaction::Transaction;

/// `ScriptVm` 是一个用于执行 Latte 脚本的虚拟机。
///
/// 脚本是由 Latte bytecode 编码的指令序列，可以在 Latte 区块链上执行。
/// `ScriptVm` 是 Latte 虚拟机的一个实现，它包含了一个解释器和一个执行环境。
///
/// 在 Latte 区块链上，每个交易都会被解码为一个 `Transaction` 对象，
/// 然后通过 `ScriptVm` 的 `execute` 方法执行。
///
/// `execute` 方法接受一个 `WorldState` 对象和一个 `Transaction` 对象作为参数，
/// 并返回一个 `Result`，其中 `Ok` 表示执行成功，`Err` 表示执行失败。
///
/// 在执行过程中，`ScriptVm` 会解码交易的 bytecode，创建一个解释器，
/// 并执行解释器的 `execute` 方法。
///
/// 如果执行成功，`execute` 方法返回 `Ok(())`；如果执行失败，`execute` 方法返回
/// `Err(StateError::VmExecutionFailed)`。
pub struct ScriptVm;

impl ScriptVm {
    pub fn new() -> Self {
        ScriptVm
    }
}

impl VmEngine for ScriptVm {
    fn execute(
        &self,
        state: &mut WorldState,
        caller: Address,
        tx: &Transaction,
    ) -> Result<(), StateError> {
        // 1. 解码 bytecode
        let code = decode_instructions(&tx.data).map_err(|_| StateError::VmExecutionFailed)?;

        // 2. 创建解释器
        let mut interpreter = Interpreter {
            state,
            caller,
            pc: 0,
            stack: Default::default(),
            gas: GasMeter::new(tx.gas_limit),
        };

        // 3. 执行
        interpreter
            .execute(&code)
            .map_err(|_| StateError::VmExecutionFailed)?;

        Ok(())
    }
}

/// `decode_instructions` 是一个用于解码 bytecode 的函数。
///
/// 输入是一个 `Vec<u8>` 类型的 bytecode，输出是一个 `Result<Vec<Instruction>, ()>` 类型的解码后的指令序列。
///
/// 该函数会遍历 bytecode 中的每个字节，并根据字节的值解析为一个 `Instruction` 对象，
/// 并将其添加到结果序列中。如果解析过程中遇到错误，比如 bytecode 格式错误、数据不足等，
/// 会返回一个 `Err(())` 结果。
///
/// 注意，解码过程中会修改输入的字节数组，因此输入参数是不可变引用，但会在解码过程中修改。
#[allow(dead_code)]
fn decode_instructions(data: &[u8]) -> Result<Vec<Instruction>, ()> {
    let mut instructions = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let op_code = data[i];
        i += 1;

        let instruction = match op_code {
            // Push 指令
            0x00 => {
                let bytes = data.get(i..i + 8).ok_or(())?.try_into().map_err(|_| ())?;
                i += 8;
                Instruction::Push(i64::from_be_bytes(bytes))
            }
            // Add 等指令，没有操作数
            0x01 => Instruction::Add,
            0x02 => Instruction::Sub,
            0x03 => Instruction::Mul,
            0x04 => Instruction::Div,
            0x05 => Instruction::Eq,
            0x06 => Instruction::Gt,
            0x07 => Instruction::Lt,
            0x08 => Instruction::Load,
            0x09 => Instruction::Store,
            // Jump 指令，下面8个字节是目标地址
            0x0A => {
                let bytes = data.get(i..i + 8).ok_or(())?.try_into().map_err(|_| ())?;
                i += 8;
                Instruction::Jump(usize::from_be_bytes(bytes))
            }
            // JumpIf指令
            0x0B => {
                let bytes = data.get(i..i + 8).ok_or(())?.try_into().map_err(|_| ())?;
                i += 8;
                Instruction::JumpIf(usize::from_be_bytes(bytes))
            }
            0x0C => Instruction::Dup,
            0x0D => Instruction::Pop,
            0x0E => Instruction::Return,
            _ => return Err(()),
        };
        instructions.push(instruction);
    }
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_instructions() {
        // 测试数据：Add(0x01), Eq(0x05), Load(0x08), Dup(0x0C), Return(0x0E)
        let data = [0x01, 0x05, 0x08, 0x0C, 0x0E];
        let instructions = decode_instructions(&data).expect("decode instructions failed");
        assert_eq!(instructions.len(), 5);
        assert_eq!(instructions[0], Instruction::Add);
        assert_eq!(instructions[1], Instruction::Eq);
        assert_eq!(instructions[2], Instruction::Load);
        assert_eq!(instructions[3], Instruction::Dup);
        assert_eq!(instructions[4], Instruction::Return);
    }
}
