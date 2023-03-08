//! Contains the core VM struct that executes bytecode

use std;
use std::io::Cursor;

use byteorder::*;
use chrono::prelude::*;
use num_cpus;
use uuid::Uuid;

use instruction::Opcode;
use std::f64::EPSILON; //Dunno why this is needed, but it is

// Need to remove these 2 constants
/// Magic number that begins every bytecode file prefix. These spell out EPIE in ASCII, if you were wondering.
pub const PIE_HEADER_PREFIX: [u8; 4] = [0x45, 0x50, 0x49, 0x45];

/// Constant that determines how long the header is. There are 60 zeros left after the prefix, for later usage if needed.
pub const PIE_HEADER_LENGTH: usize = 64;


#[derive(Clone, Debug)]
/// Enum for various types of events that can happen to the VM
pub enum VMEventType {
    Start,
    GracefulStop { code: u32 },
    Crash { code: u32 },
}

impl VMEventType {
    /// Gets the stop code of the VM, analgous to the linux exit code
    pub fn stop_code(&self) -> u32 {
        match &self {
            VMEventType::Start => 0,
            VMEventType::GracefulStop { code } => *code,
            VMEventType::Crash { code } => *code,
        }
    }
}
#[derive(Clone, Debug)]
/// Struct for a VMEvent that includes the application ID and time
pub struct VMEvent {
    pub event: VMEventType,
    at: DateTime<Utc>,
    application_id: Uuid,
}

/// Default starting size for a VM's heap
pub const DEFAULT_HEAP_STARTING_SIZE: usize = 64;

/// Default stack starting space. We'll default to 2MB.
pub const DEFAULT_STACK_SPACE: usize = 2097152;

/// Virtual machine struct that will execute bytecode
#[derive(Default, Clone)]
pub struct VM {
    /// Array that simulates having hardware registers
    pub registers: [i32; 32],
    /// Array that simulates having floating point hardware registers
    pub float_registers: [f64; 32],
    /// The bytecode of the program being run
    pub program: Vec<u8>,
    /// Number of logical cores the system reports
    pub logical_cores: usize,
    /// An alias that can be specified by the user and used to refer to the Node
    pub alias: Option<String>,
    /// Program counter that tracks which byte is being executed
    pc: usize,
    /// Keeps track of where in the stack the program currently is
    sp: usize,
    /// Keeps track of the current frame pointer
    bp: usize,
    /// Used for heap memory
    heap: Vec<u8>,
    /// Used to represent the stack
    stack: Vec<i32>,
    /// Contains the remainder of modulo division ops
    remainder: usize,
    /// Contains the result of the last comparison operation
    equal_flag: bool,
    /// Loop counter field, used with the `LOOP` instruction
    loop_counter: usize,
    /// Contains the read-only section data
    ro_data: Vec<u8>,
    /// Is a unique, randomly generated UUID for identifying this VM
    pub id: Uuid,
    /// Keeps a list of events for a particular VM
    events: Vec<VMEvent>,
    /// Server address that the VM will bind to for server-to-server communications
    server_addr: Option<String>,
    /// Port the server will bind to for server-to-server communications
    pub server_port: Option<String>,
}

impl VM {
    /// Creates and returns a new VM
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            float_registers: [0.0; 32],
            program: vec![],
            ro_data: vec![],
            heap: vec![0; DEFAULT_HEAP_STARTING_SIZE],
            stack: Vec::with_capacity(DEFAULT_STACK_SPACE),
            pc: 0,
            sp: 0,
            bp: 0,
            loop_counter: 0,
            remainder: 0,
            equal_flag: false,
            id: Uuid::new_v4(),
            alias: None,
            events: Vec::new(),
            logical_cores: num_cpus::get(),
            server_addr: None,
            server_port: None,
        }
    }

    /// Wraps execution in a loop so it will continue to run until done or there is an error
    /// executing instructions.
    pub fn run(&mut self) -> Vec<VMEvent> {
        self.events.push(VMEvent {
            event: VMEventType::Start,
            at: Utc::now(),
            application_id: self.id,
        });
        // TODO: Should setup custom errors here
        if !self.verify_header() {
            self.events.push(VMEvent {
                event: VMEventType::Crash { code: 1 },
                at: Utc::now(),
                application_id: self.id,
            });
            error!("Header was incorrect");
            return self.events.clone();
        }

        self.pc = 68 + self.get_starting_offset();
        let mut is_done = None;
        while is_done.is_none() {
            is_done = self.execute_instruction();
        }
        self.events.push(VMEvent {
            event: VMEventType::GracefulStop { code: is_done.unwrap() },
            at: Utc::now(),
            application_id: self.id,
        });
        self.events.clone()
    }

    /// Creates a VM with a specific alias
    pub fn with_alias(mut self, alias: String) -> Self {
        if alias == "" {
            self.alias = None
        } else {
            self.alias = Some(alias)
        }
        self
    }

    /// Binds a VM to a specific address and port so it can use the network
    pub fn with_cluster_bind(mut self, server_addr: String, server_port: String) -> Self {
        debug!("Binding VM to {}:{}", server_addr, server_port);
        self.server_addr = Some(server_addr);
        self.server_port = Some(server_port);
        self
    }

    /// Executes one instruction. Meant to allow for more controlled execution of the VM
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    /// Adds an arbitrary byte to the VM's program
    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }

    /// Adds an arbitrary byte to the VM's program
    pub fn add_bytes(&mut self, mut b: Vec<u8>) {
        self.program.append(&mut b);
    }

    /// Executes an instruction and returns a bool. Meant to be called by the various public run
    /// functions.
    fn execute_instruction(&mut self) -> Option<u32> {
        if self.pc >= self.program.len() {
            return Some(1);
        }
        match self.decode_opcode() {
            Opcode::STORE => {
                let register = self.next_8_bits() as usize;
                let number = i32::from(self.next_16_bits());
                self.registers[register] = number;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as usize;
            }
            Opcode::HLT => {
                info!("HLT encountered");
                return Some(0);
            }
            Opcode::IGL => {
                error!("Illegal instruction encountered");
                return Some(1);
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += value;
            }
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= value;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 == register2;
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 != register2;
                self.next_8_bits();
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 > register2;
                self.next_8_bits();
            }
            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 >= register2;
                self.next_8_bits();
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 < register2;
                self.next_8_bits();
            }
            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.equal_flag = register1 <= register2;
                self.next_8_bits();
            }
            Opcode::JMPE => {
                if self.equal_flag {
                    let register = self.next_8_bits() as usize;
                    let target = self.registers[register];
                    self.pc = target as usize;
                } else {
                    // TODO: Fix the bits
                }
            }
            Opcode::NOP => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            Opcode::INC => {
                let register_number = self.next_8_bits() as usize;
                self.registers[register_number] += 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::DEC => {
                let register_number = self.next_8_bits() as usize;
                self.registers[register_number] -= 1;
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::DJMPE => {
                let destination = self.next_16_bits();
                if self.equal_flag {
                    self.pc = destination as usize;
                } else {
                    self.next_8_bits();
                }
            }
            Opcode::PRTS => {
                // PRTS takes one operand, either a starting index in the read-only section of the bytecode
                // or a symbol (in the form of @symbol_name), which will look up the offset in the symbol table.
                // This instruction then reads each byte and prints it, until it comes to a 0x00 byte, which indicates
                // termination of the string
                let starting_offset = self.next_16_bits() as usize;
                let mut ending_offset = starting_offset;
                let slice = self.ro_data.as_slice();
                // TODO: Find a better way to do this. Maybe we can store the byte length and not null terminate? Or some form of caching where we
                // go through the entire ro_data on VM startup and find every string and its ending byte location?
                while slice[ending_offset] != 0 {
                    ending_offset += 1;
                }
                let result = std::str::from_utf8(&slice[starting_offset..ending_offset]);
                match result {
                    Ok(s) => {
                        print!("{}", s);
                    }
                    Err(e) => println!("Error decoding string for prts instruction: {:#?}", e),
                };
            }
            // Begin floating point 64-bit instructions
            Opcode::LOADF64 => {
                let register = self.next_8_bits() as usize;
                let number = f64::from(self.next_16_bits());
                self.float_registers[register] = number;
            }
            Opcode::ADDF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.float_registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUBF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.float_registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MULF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.float_registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIVF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.float_registers[self.next_8_bits() as usize] = register1 / register2;
            }
            Opcode::EQF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.equal_flag = (register1 - register2).abs() < EPSILON;
                self.next_8_bits();
            }
            Opcode::NEQF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.equal_flag = (register1 - register2).abs() > EPSILON;
                self.next_8_bits();
            }
            Opcode::GTF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.equal_flag = register1 > register2;
                self.next_8_bits();
            }
            Opcode::GTEF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.equal_flag = register1 >= register2;
                self.next_8_bits();
            }
            Opcode::LTF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.equal_flag = register1 < register2;
                self.next_8_bits();
            }
            Opcode::LTEF64 => {
                let register1 = self.float_registers[self.next_8_bits() as usize];
                let register2 = self.float_registers[self.next_8_bits() as usize];
                self.equal_flag = register1 <= register2;
                self.next_8_bits();
            }
            Opcode::SHL => {
                let reg_num = self.next_8_bits() as usize;
                let num_bits = match self.next_8_bits() {
                    0 => 16,
                    other => other,
                };
                self.registers[reg_num] = self.registers[reg_num].wrapping_shl(num_bits.into());
                self.next_8_bits();
            }
            Opcode::SHR => {
                let reg_num = self.next_8_bits() as usize;
                let num_bits = match self.next_8_bits() {
                    0 => 16,
                    other => other,
                };
                self.registers[reg_num] = self.registers[reg_num].wrapping_shr(num_bits.into());
                self.next_8_bits();
            }
            Opcode::AND => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 & register2;
            }
            Opcode::OR => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 | register2;
            }
            Opcode::XOR => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 ^ register2;
            }
            Opcode::NOT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = !register1;
                self.next_8_bits();
            }
            Opcode::LUI => {
                let register = self.next_8_bits() as usize;
                let value = self.registers[register];
                let uv1 = i32::from(self.next_8_bits());
                let uv2 = i32::from(self.next_8_bits());
                let value = value.checked_shl(8).unwrap();
                let value = value | uv1;
                let value = value.checked_shl(8).unwrap();
                let value = value | uv2;
                self.registers[register] = value;
            }
            Opcode::LOOP => {
                if self.loop_counter != 0 {
                    self.loop_counter -= 1;
                    let target = self.next_16_bits();
                    self.pc = target as usize;
                } else {
                    self.pc += 3;
                }
            }
            Opcode::CLOOP => {
                let loop_count = self.next_16_bits();
                self.loop_counter = loop_count as usize;
                self.next_8_bits();
            }
            Opcode::LOADM => {
                let offset = self.registers[self.next_8_bits() as usize] as usize;
                let data: i32;
                // Explicit scoping is necessary here because we do an immutable borrow of self, then a mutable borrow to assign the result
                {
                    let mut slice = &self.heap[offset..offset + 4];
                    data = slice.read_i32::<LittleEndian>().unwrap();
                }
                self.registers[self.next_8_bits() as usize] = data;
            }
            Opcode::SETM => {
                let _offset = self.registers[self.next_8_bits() as usize] as usize;
                let data = self.registers[self.next_8_bits() as usize];
                let mut buf: [u8; 4] = [0, 0, 0, 0];
                let _ = buf.as_mut().write_i32::<LittleEndian>(data);
            }
            Opcode::PUSH => {
                let data = self.registers[self.next_8_bits() as usize];
                self.stack.push(data);
                self.sp = self.sp + 1;
            }
            Opcode::POP => {
                let target_register = self.next_8_bits() as usize;
                self.registers[target_register] = self.stack.pop().unwrap();
                self.sp = self.sp - 1
            }
            Opcode::CALL => {
                // First we capture the return destination for when the function is done
                let return_destination = self.pc + 3;
                // Next we get the address we are going to jump to, i.e., that of the
                // destination subroutine
                let destination = self.next_16_bits();
                // Push the return address onto the stack
                self.stack.push(return_destination as i32);
                self.stack.push(self.bp as i32);
                self.bp = self.sp;
                // Change the program counter to that of the destination
                self.pc = destination as usize;
            }
            Opcode::RET => {
                self.sp = self.bp;
                self.bp = self.stack.pop().unwrap() as usize;
                self.pc = self.stack.pop().unwrap() as usize;
            }
        };
        None
    }

    pub fn print_i32_register(&self, register: usize) {
        let bits = self.registers[register];
        println!("bits: {:#032b}", bits);
    }

    pub fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm.float_registers[0] = 5.0;
        test_vm.float_registers[1] = 10.0;
        test_vm
    }

    pub fn prepend_header(mut b: Vec<u8>) -> Vec<u8> {
        let mut prepension = vec![];
        for byte in &PIE_HEADER_PREFIX {
            prepension.push(byte.clone());
        }

        // The 4 is added here to allow for the 4 bytes that tell the VM where the executable code starts
        while prepension.len() < PIE_HEADER_LENGTH + 4 {
            prepension.push(0);
        }

        prepension.append(&mut b);
        prepension
    }

    fn get_starting_offset(&self) -> usize {
        let mut rdr = Cursor::new(&self.program[64..68]);
        rdr.read_u32::<LittleEndian>().unwrap() as usize
    }

    // Attempts to decode the byte the VM's program counter is pointing at into an opcode
    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn _i32_to_bytes(num: i32) -> [u8; 4] {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        buf.as_mut().write_i32::<LittleEndian>(num).unwrap();
        buf
    }

    // Attempts to decode the next byte into an opcode
    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    // Grabs the next 16 bits (2 bytes)
    fn next_16_bits(&mut self) -> u16 {
        let result = ((u16::from(self.program[self.pc])) << 8) | u16::from(self.program[self.pc + 1]);
        self.pc += 2;
        result
    }

    // Processes the header of bytecode the VM is asked to execute
    fn verify_header(&self) -> bool {
        if self.program[0..4] != PIE_HEADER_PREFIX {
            return false;
        }
        true
    }
}