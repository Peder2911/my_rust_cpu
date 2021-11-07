struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn new() -> Self {
        CPU {
            memory: [0; 4096],
            registers: [0; 16],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    fn program(&mut self, address: usize, opcode: u16){
        self.memory[address] = (opcode >> 8) as u8;
        self.memory[(address)+1] = opcode as u8;
    }

    fn call(&mut self, addr: u16){
        println!("Calling 0x{:04x}",addr);

        if self.stack_pointer > self.stack.len() {
            panic!("Stack overflow!!")
        }

        self.stack[self.stack_pointer] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self){
        println!("Returning from 0x{:04x} to 0x{:04x}",
                 self.position_in_memory,
                 self.stack[self.stack_pointer]
             );
        if self.stack_pointer == 0 {
            panic!("Stack underflow!!")
        }

        self.stack_pointer -= 1;
        self.position_in_memory = self.stack[self.stack_pointer] as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        println!("Adding {} and {}",x,y);
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow_detected) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow_detected {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn read_opcode(&self) -> u16 {
        let op_byte1 = self.memory[self.position_in_memory] as u16;
        let op_byte2 = self.memory[self.position_in_memory + 1] as u16;
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            println!("At 0x{:04x}",self.position_in_memory);
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            // High Byte
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;

            // Low Byte
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F)) as u8;

            let nnn = opcode & 0x0FFF;

            match (c,x,y,d) {
                (  0,  0,  0,  0) => { return; }
                (0x8,  _,  _,0x4) => self.add_xy(x,y),
                (  0,  0,0xE,0xE) => self.ret(),
                (0x2,  _,  _,  _) => self.call(nnn),
                _ => todo!("opcode 0x{:04x}", opcode)
            }
        }
    }
}

fn main() {
    let mut cpu = CPU::new();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.program(0x000,0x2100);
    cpu.program(0x002,0x2100);
    cpu.program(0x004,0x2100);
    cpu.program(0x100,0x8014);
    cpu.program(0x102,0x8014);
    cpu.program(0x104,0x00EE);

    cpu.run();
    assert_eq!(cpu.registers[0], 65);
    println!("Result of operations: {}",cpu.registers[0]);
}
