use std::fs;

// position class/struct with position X = horizontal position ,Y = Vertical Postion
pub struct postion {
    X: u32,
    Y: u32,
    aim: u32,
}

impl postion {
    fn area(&self) -> u32 {
        self.X * self.Y
    }
}

pub fn move_sub(instructionList: Vec<instruction>, mut pos: postion) -> postion {
    let iterInstruct = instructionList.iter();

    for val in iterInstruct {
        if val.action == "forward" {
            pos.X += val.units;
            pos.Y += pos.aim * val.units;
            //println!("forward");
        } else if val.action == "down" {
            pos.aim += val.units;
            //println!("down");
        } else if val.action == "up" {
            pos.aim -= val.units;
            //println!("up");
        }

        //println!("{} {}", val.action, val.units);
    }
    pos
}

pub struct instruction {
    action: String,
    units: u32,
}

pub fn build_instruction(action: String, units: String) -> instruction {
    instruction {
        action: action,
        units: units.parse::<u32>().unwrap(),
    }
}

pub fn build_pos(X: u32, Y: u32, aim: u32) -> postion {
    postion { X, Y, aim }
}

pub fn build_instruction_list<'a>(content: &'a str) -> Vec<instruction> {
    let mut instructionList: Vec<instruction> = Vec::new();

    for line in content.lines() {
        let newLine: Vec<&str> = line.split(" ").collect();
        //println!("{} {}", newLine[0], newLine[1]);
        let newInstruction: instruction =
            build_instruction(newLine[0].to_string(), newLine[1].to_string());
        instructionList.push(newInstruction);
    }
    instructionList
}

fn main() {
    let filename = "../input.txt";

    let content = fs::read_to_string(filename).expect("Okay thre is no file there.");
    let instructionList: Vec<instruction> = build_instruction_list(&content);
    let mut pos: postion = build_pos(0, 0, 0);
    println!("{} {}", pos.X, pos.Y);

    pos = move_sub(instructionList, pos);
    println!("{} {}", pos.X, pos.Y);
    let area: u32 = pos.area();
    println!("{}", area);
}
