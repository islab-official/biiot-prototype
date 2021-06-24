pub type OpCode = u8;

//0x00Range-Arithmeticoperations
pub const STOP:     OpCode = 0x00;
pub const ADD:      OpCode = 0x01;
pub const MUL:      OpCode = 0x02;
pub const SUB:      OpCode = 0x03;
pub const DIV:      OpCode = 0x04;
pub const SDIV:     OpCode = 0x05;
pub const MOD:      OpCode = 0x06;
pub const SMOD:     OpCode = 0x07;
pub const ADDMOD:   OpCode = 0x08;
pub const MULMOD:   OpCode = 0x09;
pub const EXP:      OpCode = 0x0A;

//0x10Range-Comparisonoperations(with0x20,SHA3operation)
pub const LT:       OpCode = 0x10;
pub const GT:       OpCode = 0x11;
pub const SLT:      OpCode = 0x12;
pub const SGT:      OpCode = 0x13;
pub const EQ:       OpCode = 0x14;
pub const ISZERO:   OpCode = 0x15;
pub const AND:      OpCode = 0x16;
pub const OR:       OpCode = 0x17;
pub const XOR:      OpCode = 0x18;
pub const NOT:      OpCode = 0x19;
pub const BYTE:     OpCode = 0x1A;
pub const SHL:      OpCode = 0x1B;
pub const SHR:      OpCode = 0x1C;
pub const SAR:      OpCode = 0x1D;
pub const SHA3:     OpCode = 0x20;

//0x30Range-Closurestateoperations
pub const ADDRESS:          OpCode = 0x30;
pub const BALANCE:          OpCode = 0x31;
pub const ORIGIN:           OpCode = 0x32;
pub const CALLER:           OpCode = 0x33;
pub const CALLVALUE:        OpCode = 0x34;
pub const CALLDATALOAD:     OpCode = 0x35;
pub const CALLDATASIZE:     OpCode = 0x36;
pub const CALLDATACOPY:     OpCode = 0x37;
pub const CODESIZE:         OpCode = 0x38;
pub const CODECOPY:         OpCode = 0x39;
pub const GASPRICE:         OpCode = 0x3A;
pub const EXTCODESIZE:      OpCode = 0x3B;
pub const EXTCODECOPY:      OpCode = 0x3C;
pub const RETURNDATASIZE:   OpCode = 0x3D;
pub const RETURNDATACOPY:   OpCode = 0x3E;
pub const EXTCODEHASH:      OpCode = 0x3F;

//0x40Range-Blockoperations
pub const BLOCKHASH:    OpCode = 0x40;
pub const COINBASE:     OpCode = 0x41;
pub const TIMESTAMP:    OpCode = 0x42;
pub const NUMBER:       OpCode = 0x43;
pub const DIFFICULTY:   OpCode = 0x44;
pub const GASLIMIT:     OpCode = 0x45;
pub const CHAINID:      OpCode = 0x46;
pub const SELFBALANCE:  OpCode = 0x47;

//0x50Range-storageandexecutionoperations
pub const POP:          OpCode = 0x50;
pub const MLOAD:        OpCode = 0x51;
pub const MSTORE:       OpCode = 0x52;
pub const MSTORE8:      OpCode = 0x53;
pub const SLOAD:        OpCode = 0x54;
pub const SSTORE:       OpCode = 0x55;
pub const JUMP:         OpCode = 0x56;
pub const JUMPI:        OpCode = 0x57;
pub const PC:           OpCode = 0x58;
pub const MSIZE:        OpCode = 0x59;
pub const GAS:          OpCode = 0x5A;
pub const JUMPDEST:     OpCode = 0x5B;
pub const BEGINSUB:     OpCode = 0x5C;
pub const RETURNSUB:    OpCode = 0x5D;
pub const JUMPSUB:      OpCode = 0x5E;

//0x60Range
pub const PUSH1:    OpCode = 0x60;
pub const PUSH2:    OpCode = 0x61;
pub const PUSH3:    OpCode = 0x62;
pub const PUSH4:    OpCode = 0x63;
pub const PUSH5:    OpCode = 0x64;
pub const PUSH6:    OpCode = 0x65;
pub const PUSH7:    OpCode = 0x66;
pub const PUSH8:    OpCode = 0x67;
pub const PUSH9:    OpCode = 0x68;
pub const PUSH10:   OpCode = 0x69;
pub const PUSH11:   OpCode = 0x6A;
pub const PUSH12:   OpCode = 0x6B;
pub const PUSH13:   OpCode = 0x6C;
pub const PUSH14:   OpCode = 0x6D;
pub const PUSH15:   OpCode = 0x6E;
pub const PUSH16:   OpCode = 0x6F;
pub const PUSH17:   OpCode = 0x70;
pub const PUSH18:   OpCode = 0x71;
pub const PUSH19:   OpCode = 0x72;
pub const PUSH20:   OpCode = 0x73;
pub const PUSH21:   OpCode = 0x74;
pub const PUSH22:   OpCode = 0x75;
pub const PUSH23:   OpCode = 0x76;
pub const PUSH24:   OpCode = 0x77;
pub const PUSH25:   OpCode = 0x78;
pub const PUSH26:   OpCode = 0x79;
pub const PUSH27:   OpCode = 0x7A;
pub const PUSH28:   OpCode = 0x7B;
pub const PUSH29:   OpCode = 0x7C;
pub const PUSH30:   OpCode = 0x7D;
pub const PUSH31:   OpCode = 0x7E;
pub const PUSH32:   OpCode = 0x7F;
pub const DUP1:     OpCode = 0x80;
pub const DUP2:     OpCode = 0x81;
pub const DUP3:     OpCode = 0x82;
pub const DUP4:     OpCode = 0x83;
pub const DUP5:     OpCode = 0x84;
pub const DUP6:     OpCode = 0x85;
pub const DUP7:     OpCode = 0x86;
pub const DUP8:     OpCode = 0x87;
pub const DUP9:     OpCode = 0x88;
pub const DUP10:    OpCode = 0x89;
pub const DUP11:    OpCode = 0x8A;
pub const DUP12:    OpCode = 0x8B;
pub const DUP13:    OpCode = 0x8C;
pub const DUP14:    OpCode = 0x8D;
pub const DUP15:    OpCode = 0x8E;
pub const DUP16:    OpCode = 0x8F;
pub const SWAP1:    OpCode = 0x90;
pub const SWAP2:    OpCode = 0x91;
pub const SWAP3:    OpCode = 0x92;
pub const SWAP4:    OpCode = 0x93;
pub const SWAP5:    OpCode = 0x94;
pub const SWAP6:    OpCode = 0x95;
pub const SWAP7:    OpCode = 0x96;
pub const SWAP8:    OpCode = 0x97;
pub const SWAP9:    OpCode = 0x98;
pub const SWAP10:   OpCode = 0x99;
pub const SWAP11:   OpCode = 0x9A;
pub const SWAP12:   OpCode = 0x9B;
pub const SWAP13:   OpCode = 0x9C;
pub const SWAP14:   OpCode = 0x9D;
pub const SWAP15:   OpCode = 0x9E;
pub const SWAP16:   OpCode = 0x9F;

//0x A0 range - logging ops
pub const LOG0: OpCode = 0xA0;
pub const LOG1: OpCode = 0xA1;
pub const LOG2: OpCode = 0xA2;
pub const LOG3: OpCode = 0xA3;
pub const LOG4: OpCode = 0xA4;

//unofficial opcodes used for parsing
//0xF0 range - closures operations
pub const CREATE:       OpCode = 0xF0;
pub const CALL:         OpCode = 0xF1;
pub const CALLCODE:     OpCode = 0xF2;
pub const RETURN:       OpCode = 0xF3;
pub const DELEGATECALL: OpCode = 0xF4;
pub const CREATE2:      OpCode = 0xF5;
pub const STATICCALL:   OpCode = 0xFA;
pub const REVERT:       OpCode = 0xFD;
pub const SELFDESTRUCT: OpCode = 0xFF;

pub fn opcode_to_u8(_opcode: OpCode) -> u8 {
    _opcode as u8
}

pub fn u8_to_opcode(_v: u8) -> OpCode {
    _v as OpCode
}

pub fn get_opcode_name(_code: &u8) -> &str {
    return match _code {
        0x00 => "STOP",
        0x01 => "ADD",
        0x02 => "MUL",
        0x03 => "SUB",
        0x04 => "DIV",
        0x05 => "SDIV",
        0x06 => "MOD",
        0x07 => "SMOD",
        0x08 => "ADD_MOD",
        0x09 => "MUL_MOD",
        0x0A => "EXP",

        0x10 => "LT",
        0x11 => "GT",
        0x12 => "SLT",
        0x13 => "SGT",
        0x14 => "EQ",
        0x15 => "IS_ZERO",
        0x16 => "AND",
        0x17 => "OR",
        0x18 => "XOR",
        0x19 => "NOT",
        0x1A => "BYTE",
        0x1B => "SHL",
        0x1C => "SHR",
        0x1D => "SAR",
        0x20 => "SHA3",

        0x30 => "ADDRESS",
        0x31 => "BALANCE",
        0x32 => "ORIGIN",
        0x33 => "CALLER",
        0x34 => "CALLVALUE",
        0x35 => "CALLDATA_LOAD",
        0x36 => "CALLDATA_SIZE",
        0x37 => "CALL_DATA_COPY",
        0x38 => "CODE_SIZE",
        0x39 => "CODE_COPY",
        0x3A => "GAS_PRICE",
        0x3B => "EXTCODE_SIZE",
        0x3C => "EXTCODE_COPY",
        0x3D => "RETURNDATA_SIZE",
        0x3E => "RETURNDATA_COPY",
        0x3F => "EXTCODEHASH",

        0x40 => "BLOCKHASH",
        0x41 => "COINBASE",
        0x42 => "TIMESTAMP",
        0x43 => "NUMBER",
        0x44 => "DIFFICULTY",
        0x45 => "GAS_LIMIT",
        0x46 => "CHAIN_ID",
        0x47 => "SELF_BALANCE",

        0x50 => "POP",
        0x51 => "MLOAD",
        0x52 => "MSTORE",
        0x53 => "MSTORE8",
        0x54 => "SLOAD",
        0x55 => "SSTORE",
        0x56 => "JUMP",
        0x57 => "JUMPI",
        0x58 => "PC",
        0x59 => "MSIZE",
        0x5A => "GAS",
        0x5B => "JUMPDEST",
        0x5C => "BEGINSUB",
        0x5D => "RETURNSUB",
        0x5E => "JUMPSUB",

        0x60 => "PUSH1",
        0x61 => "PUSH2",
        0x62 => "PUSH3",
        0x63 => "PUSH4",
        0x64 => "PUSH5",
        0x65 => "PUSH6",
        0x66 => "PUSH7",
        0x67 => "PUSH8",
        0x68 => "PUSH9",
        0x69 => "PUSH10",
        0x6A => "PUSH11",
        0x6B => "PUSH12",
        0x6C => "PUSH13",
        0x6D => "PUSH14",
        0x6E => "PUSH15",
        0x6F => "PUSH16",
        0x70 => "PUSH17",
        0x71 => "PUSH18",
        0x72 => "PUSH19",
        0x73 => "PUSH20",
        0x74 => "PUSH21",
        0x75 => "PUSH22",
        0x76 => "PUSH23",
        0x77 => "PUSH24",
        0x78 => "PUSH25",
        0x79 => "PUSH26",
        0x7A => "PUSH27",
        0x7B => "PUSH28",
        0x7C => "PUSH29",
        0x7D => "PUSH30",
        0x7E => "PUSH31",
        0x7F => "PUSH32",

        0x80 => "DUP1",
        0x81 => "DUP2",
        0x82 => "DUP3",
        0x83 => "DUP4",
        0x84 => "DUP5",
        0x85 => "DUP6",
        0x86 => "DUP7",
        0x87 => "DUP8",
        0x88 => "DUP9",
        0x89 => "DUP10",
        0x8A => "DUP11",
        0x8B => "DUP12",
        0x8C => "DUP13",
        0x8D => "DUP14",
        0x8E => "DUP15",
        0x8F => "DUP16",

        0x90 => "SWAP1",
        0x91 => "SWAP2",
        0x92 => "SWAP3",
        0x93 => "SWAP4",
        0x94 => "SWAP5",
        0x95 => "SWAP6",
        0x96 => "SWAP7",
        0x97 => "SWAP8",
        0x98 => "SWAP9",
        0x99 => "SWAP10",
        0x9A => "SWAP11",
        0x9B => "SWAP12",
        0x9C => "SWAP13",
        0x9D => "SWAP14",
        0x9E => "SWAP15",
        0x9F => "SWAP16",

        0xA0 => "LOG0",
        0xA1 => "LOG1",
        0xA2 => "LOG2",
        0xA3 => "LOG3",
        0xA4 => "LOG4",

        // unofficial opcodes used for parsing
        0xF0 => "CREATE",
        0xF1 => "CALL",
        0xF2 => "CALLCODE",
        0xF3 => "RETURN",
        0xF4 => "DELEGATE_CALL",
        0xF5 => "CREATE2",

        0xFA => "STATIC_CALL",
        0xFD => "REVERT",
        0xFF => "SELF_DESTRUCT",
        _ => "NONE"
    }
}