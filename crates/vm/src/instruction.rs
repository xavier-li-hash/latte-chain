
/// `Instruction` 是 Latte 虚拟机的指令枚举类型。
///
/// 每个指令都是一个具体的操作，可以在 Latte 虚拟机中执行。Latte 虚拟机通过解析 bytecode 并将其解码为一系列 `Instruction` 对象来执行合约代码。
///
/// 目前 Latte 虚拟机支持的指令包括如下几种：
///
/// - `Push`：将一个 `i64` 类型的数值压入栈顶。
/// - `Add`：将栈顶的两个数值相加并将结果压入栈顶。
/// - `Sub`：将栈顶的两个数值相减并将结果压入栈顶。
/// - `Mul`：将栈顶的两个数值相乘并将结果压入栈顶。
/// - `Div`：将栈顶的两个数值相除并将结果压入栈顶。
/// - `Eq`：将栈顶的两个数值进行相等性比较，并将结果压入栈顶。
/// - `Gt`：将栈顶的两个数值进行大于性比较，并将结果压入栈顶。
/// - `Lt`：将栈顶的两个数值进行小于性比较，并将结果压入栈顶。
/// - `Load`：将栈顶的内容作为key获取账户的storage
/// - `Store`：存储栈顶的值到账户中
/// - `Return`：结束当前函数的执行并将结果压入调用者的栈顶。
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Instruction {
    Push(i64),

    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Gt,
    Lt,

    Load,  // 从 account.storage 读
    Store, // 写入 account.storage

    Jump(usize),
    JumpIf(usize),

    Dup,
    Pop,

    Return,
}
