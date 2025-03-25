// YOU SHOULD JUST COPY OVER YOUR SOLUTION
// FROM PART 1 AND USE THAT AS THE START OF PART 2.
use std::{fs, str::FromStr};

static INPUT_FILE: &str = "input.txt";

fn main() {
    let contents =
        fs::read_to_string(INPUT_FILE).expect(&format!("Failed to open file '{INPUT_FILE}'"));

    // This splits the input into two parts, the text before the blank
    // line (`stack_config`) and the part after the blank line (`instructions`).
    let (stack_config, instructions) = contents
        .split_once("\n\n")
        .expect("There was no blank line in the input");

    // The `.parse()` call actually calls the appropriate `from_str()`, which
    // in this case is in the `impl FromStr for Stacks` block.
    let stacks: Stacks = stack_config
        .parse()
        .expect("Failed to parse stack configuration");

    // This `.parse()` call uses the implementation of `from_str()`
    // in the `impl FromStr for CraneInstructions` block.
    let instructions: CraneInstructions = instructions
        .parse()
        .expect("Failed to parse crane instructions");

    // Run all the instructions, returning the final `Stacks` state.
    let final_state = stacks
        .apply_instructions(&instructions)
        .expect("Applying an instruction set failed");

    // Get the top of the stacks and print that out.
    println!(
        "The top of the stacks is {}",
        final_state
            .tops_string()
            .expect("Tried to take the top of an empty stack")
    );
}

#[derive(Debug)]
pub enum ParseError {
    // Add different variants as you discover different kinds of parsing errors.
    // This could include things like too many stacks, illegal strings on a stack, etc.
    NullError, ParseIntError
}

const NUM_STACKS: usize = 9;

#[derive(Debug, Default)]
pub struct Stacks {
    stacks: [Stack; NUM_STACKS],
}

#[derive(Debug)]
enum CraneError {
    // Add different variants as you discover different kinds of errors
    // that can occur when applying a crane instruction.
    // This could include things like trying to move from an empty stack,
    // trying to get the top of an empty stack, etc.
    EmptyStack
}

impl Stacks {
    /// Apply a single instruction to the set of stacks in `self`.
    /// Return the new set of stacks, or a `CraneError` if the instruction
    /// is invalid.
    fn apply_instruction(mut self, instruction: &CraneInstruction) -> Result<Self, CraneError> {
        let mut moving_stack: Vec<char> = Vec::new();
        for _i in 0..instruction.num_to_move {
            let temp = self.stacks[instruction.from_stack - 1].stack.pop();
            //print!("{:?}\n", temp);
            moving_stack.push(temp.expect("Push to moving stack failed"));
        }
        /*
        PART TWO SOLUTION
        In order to preserve the order in which crates are taken out of the stack we introduce moving_stack. Once all the crates that are
        moving is added our moving stack, we pop/push to the new stack location. This method maintains the order of the crates.
         */
        for _i in 0..moving_stack.len() {
            let temp = moving_stack.pop();

            self.stacks[instruction.to_stack - 1].stack.push(temp.expect("Push to new stack failed"));
        }
        Ok(self)
    }

    /// Perform each of these instructions in order on the set of stacks
    /// in `self`. Return the new set of stacks, or a `CraneError` if
    /// any of the instructions are invalid.
    fn apply_instructions(mut self, instructions: &CraneInstructions) -> Result<Self, CraneError> {
        for instruction in &instructions.instructions {
            self = self.apply_instruction(instruction)?;
        }
        Ok(self)
    }

    /// Return a string containing the top character of each stack in order.
    /// The stacks should all be non-empty; if any is empty return a `CraneError`.
    fn tops_string(&self) -> Result<String, CraneError> {
        let mut temp_string: String = "".to_string();
        for stack in &self.stacks {
            if stack.len() == 0 {
                return Err(CraneError::EmptyStack);
            }
            temp_string = temp_string + &stack.stack.last().unwrap().to_string();
        }
        Ok::<String, CraneError>(temp_string)
    }
}

impl FromStr for Stacks {
    type Err = ParseError;

    // You probably want to use `s.lines()` to create an iterator over the lines (one per stack).
    // Then for each line:
    //   (a) extract the number at the front as the stack number
    //   (b) extract the following characters as the stack contents
    // The function `split_ascii_whitespace()` should prove useful.
    // Note that the stack numbers start at 1 and you'll need the indices
    // in `Stacks::stacks` to start at 0.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks: [Stack; 9] = Default::default();
        for line in s.lines() {
            let mut i = line.split_ascii_whitespace();
            let index_str = i.next().ok_or(ParseError::NullError)?; // Checks if vec is something...? COME BACK TO THIS COMMENT
            let index = index_str.parse::<usize>().map_err(|_| ParseError::ParseIntError)? - 1;
            

            let stack_elements : Vec<char> = i.map(|s| s.chars().next().expect("IT'S BROKEN ")).collect(); 
            // print!("{:?}", stack_elements);
            stacks[index] = Stack { stack: stack_elements};
            
        }
        //print!("\n stacks: {:?}\n", stacks);
        // todo!()
        Ok(Stacks { stacks: stacks })
    }
}

#[derive(Debug, Default)]
pub struct Stack {
    stack: Vec<char>,
}

impl Stack {
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl FromStr for Stack {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

// Implementing `PartialEq<Vec<char>> for Stack` here allows us to
// say things like `vec!['A', 'B', 'C'] == stack`. This is useful
// for testing, where we might want to compare a `Stack` to a `Vec<char>`
// using something like ``assert_eq!(stack, vec!['A', 'B', 'C'])`.
impl PartialEq<Vec<char>> for Stack {
    fn eq(&self, other: &Vec<char>) -> bool {
        self.stack == *other
    }
}

struct CraneInstruction {
    num_to_move: usize,
    from_stack: usize,
    to_stack: usize,
}

impl FromStr for CraneInstruction {
    type Err = ParseError;

    // The instruction specification lines have the form
    //     move 13 from 8 to 7
    // All we need to capture are the three numbers, which happen to
    // be in the odd positions in the input line. I used a `filter` statement
    // to extract those three items from the list, which I could
    // then parse into `usize` using a `map` statement. You could also just
    // "reach" into the split string directly if you find that easier.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s.split_ascii_whitespace();
        // print!("{:?}", i);
        let numbers: Vec<usize> = i.map(|s| s.parse::<usize>()).filter(|s| s.is_ok()).map(|s|s.unwrap()).collect();
        // print!("{:?}\n", numbers[0]);
        // print!("{:?}\n", numbers[1]);
        // print!("{:?}\n", numbers[2]);
        Ok(CraneInstruction { num_to_move: numbers[0], from_stack: numbers[1], to_stack: numbers[2] })
    }
}

struct CraneInstructions {
    instructions: Vec<CraneInstruction>,
}

impl FromStr for CraneInstructions {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instruction_vec: Vec<CraneInstruction> = vec![];
        for line in s.lines() {
            if line == "" {
                break
            }
            instruction_vec.push(CraneInstruction::from_str(line).unwrap());
        }

        Ok(CraneInstructions {instructions: instruction_vec})
    }
}
