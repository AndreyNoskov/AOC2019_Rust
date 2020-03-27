use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    // The first part
    let mut computer = Computer { registers: vec!(), position: 0 };
    for x in contents.clone().split(",") {
        computer.load_instruction(x.parse().unwrap());
    }

    // preinstalled conditions
    computer.registers[1] = 12;
    computer.registers[2] = 2;

    let mut condition: i64;
    loop {
        condition = computer.next();
        if condition == 99 {
            break;
        }
    }
    println!("The first part answer is {}", computer.registers[0]);

    // The second part
    'out: for noun in 0..100 {
        for verb in 0..100 {
            let mut tmp_computer: Computer = Computer {registers: vec!(), position: 0};
            for x in contents.clone().split(",") {
                tmp_computer.load_instruction(x.parse().unwrap());
            }
            tmp_computer.registers[1] = noun;
            tmp_computer.registers[2] = verb;
            let mut tmp_condition: i64;
            loop {
                tmp_condition = tmp_computer.next();
                if tmp_condition == 99 {
                    break;
                }
            }
            if tmp_computer.registers[0] == 19690720 {
                println!("The second part answer is {}", 100 * noun + verb);
                break 'out;
            }
        }
    }
}

struct Computer {
    registers: Vec<i64>,
    position: usize,
}

impl Computer {
    fn load_instruction(&mut self, x: i64) {
        self.registers.push(x);
    }

    fn next(&mut self) -> i64 {
        let op_code: i64 = self.registers[self.position].clone();
        if op_code == 1 || op_code == 2 {
            let value_1: usize = self.registers[self.position + 1] as usize;
            let value_2: usize = self.registers[self.position + 2] as usize;
            let value_3: usize = self.registers[self.position + 3] as usize;
            if op_code == 1 {
                self.registers[value_3] = self.registers[value_1] + self.registers[value_2];
            } else if op_code == 2 {
                self.registers[value_3] = self.registers[value_1] * self.registers[value_2];
            }
            self.position += 4;
        } else if op_code == 99 {
        } else {
            unreachable!()
        }
        self.registers[self.position]
    }
}
