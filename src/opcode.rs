#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    STOP,       // 0x00: Halts execution
    ADD,        // 0x01: Add top two stack items
    MUL,        // 0x02: Multiple top two stack items
    SUB,        // 0x03: Subtract top two stack items
    DIV,        // 0x04: Divide top two stack items
    SDIV,       // 0x05: Sign divide top two stack items
    MOD,        // 0x06: Modular remainder top two stack items
    SMOD,       // 0x07: Sign modular remainder top two stack items
    ADDMOD,     // 0x08: Modular addition top three stack items ((a + b) % N)
    MULMOD,     // 0x08: Modular multiple top three stack items ((a * b) % N)
    EXP,        // 0x0A: Exponent top two stack items (a^b)
    SIGNEXTEND, // 0x0B: Extend length of two’s complement signed integer
    LT,         // 0x10: Less than comparsion top two stack items
    GT,         // 0x11: Greater than comparsion top two stack items
    SLT,        // 0x12: Signed less than comparsion top two stack items
    SGT,        // 0x13: Signed greater than comparsion top two stack items
    EQ,         // 0x14: Equal to comparsion top two stack items
    ISZERO,     // 0x15: Zero comparsion top stack item
    AND,        // 0x16: Bitwise AND top two stack items
    OR,         // 0x17: Bitwise OR top two stack items
    XOR,        // 0x18: Bitwise XOR top two stack items
    NOT,        // 0x19: Bitwise NOT top stack item
    BYTE,       // 0x1A: Retrieve single byte from word top two stack items
    SHL,        // 0x1B: Left shift operation top two stack items
    SHR,        // 0x1C: Right shift operation top two stack items
    SAR,        // 0x1D: Arithmetic (signed) right shift top two stack items
    PUSH1,      // 0x60: Place 1 byte on top of stack
    PUSH2,      // 0x61: Place 2 byte on top of stack
    PUSH3,      // 0x62: Place 3 byte on top of stack
    PUSH32,     // 0x7F: Place 32 byte on top of stack
}
