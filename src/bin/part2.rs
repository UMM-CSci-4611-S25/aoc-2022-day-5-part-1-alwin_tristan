// YOU SHOULD JUST COPY OVER YOUR SOLUTION
// FROM PART 1 AND USE THAT AS THE START OF PART 2.

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_apply_instruction() {
    let stacks = Stacks {
        stacks: [
            Stack {
                stack: vec!['A', 'B', 'C'],
            },
            Stack {
                stack: vec!['D', 'E', 'F'],
            },
            Stack {
                stack: vec!['G', 'H', 'I'],
            },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
        ],
    };

    let instruction = CraneInstruction {
        num_to_move: 2,
        from_stack: 1,
        to_stack: 2,
    };

    let new_stacks = stacks
        .apply_instruction(&instruction)
        .expect("Failed to apply instruction");

    assert_eq!(new_stacks.stacks[0], vec!['A']);
    assert_eq!(new_stacks.stacks[1], vec!['D', 'E', 'F', 'B', 'C']);
}

#[test]
fn given_example() {
    let stacks = Stacks {
        stacks: [
            Stack {
                stack: vec!['Z', 'N'],
            },
            Stack {
                stack: vec!['M', 'C', 'D'],
            },
            Stack {
                stack: vec!['P'],
            },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
            Stack { stack: Vec::new() },
        ],
    };

    let instruction = CraneInstruction {
        num_to_move: 1,
        from_stack: 2,
        to_stack: 1
    }

    let new_stacks = stacks
        .apply_instruction(&instruction)
        .expect("Failed to apply instruction");

    assert_eq!(new_stacks.stacks[0],vec!['Z','N','D']);
    assert_eq!(new_stacks.stacks[1],vec!['M','C']);
    assert_eq!(new_stacks.stacks[1],vec!['P']);
}

#[test]
fn partial_test_part2() {
// 1 J H P M S F N V M V T P F B Q N 
// 2 S R L M J D Q
// 3 N Q D H 
// 4 R S C L R
// 5 
// 6 T R 
// 7 G V 
// 8 C Z S P D L R C S W B C
// 9 D S J V G P B F

// 11 12 13 14 15 16

// instructions = """
// move 1 from 8 to 4
// move 1 from 7 to 8
// move 1 from 6 to 3
// move 2 from 6 to 5
// move 8 from 5 to 1
// move 5 from 3 to 8
// """

}