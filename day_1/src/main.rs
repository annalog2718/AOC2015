use std::{collections::HashMap, fs};
fn main() {
    let mut floor = 0;
    let mut step_count = 0;
    for line in fs::read_to_string("src/input.txt").unwrap().lines()
    {
        for step in line.chars()
        {
            if step == '('
            {
                floor += 1;
            }
            else if step == ')'
            {
                floor -= 1;
            }

            step_count += 1;
            if floor == -1
            {
                println!("{}", step_count);
            }
        }
    }

    println!("{}", floor);
}
