use crate::Colorize;
use super::{events::VMEvent, bytecode::Instruction};

/// Virtual CPU implementation, only support int 32 bits.
#[derive(Default)]
pub struct VirtualCPU {
    pub pc: usize, // program counter, holds the address of the next instruction to be executed
    pub registers: [u32; 32], // 32 registers (the first register is the zero register /!\ read-only)
    pub cmp_register: u8, // Store the result of the lost comparison
    pub memory: Vec<u8>, // Memory that behave as a stack and a heap for the virtual machine the first part of it is the data section and the second part is the dynamic memory (stack/heap)
    pub program: Vec<Instruction>, // Representation of the program instructions
    pub base_pointer: usize, // Base pointer for the current stack frame
    pub frame_pointer: usize, // Frame pointer for the current stack frame
    pub stack_pointer: usize, // Stack pointer for the current stack frame
}

impl VirtualCPU {
    pub fn new() -> VirtualCPU {
        VirtualCPU {
            pc: 0,
            registers: [0; 32],
            cmp_register: 0,
            memory: vec![],
            program: vec![],
            base_pointer: 0,
            frame_pointer: 0,
            stack_pointer: 0,
        }
    }
    pub fn run(&mut self) {

    }
}
