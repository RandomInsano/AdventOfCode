#![allow(non_snake_case)]

mod data;
use data::DATA;

fn main() {
    let mut current_calories = 0;
    let mut current_elf_index = 1;

    let mut richest_elf_index = 0;
    let mut calorie_high_score = 0;

    for line in DATA.split('\n') {
        match line.parse::<u32>() {
            Ok(x) => {
                current_calories += x;
            },
            Err(_) => {
                print!("Elf {current_elf_index} had {current_calories}");

                if current_calories > calorie_high_score {
                    println!(" New high score!");
                    richest_elf_index = current_elf_index;
                    calorie_high_score = current_calories;
                } else {
                    println!();
                }

                current_elf_index += 1;
                current_calories = 0;
            }
        }
    }

    println!("Richest elf is {richest_elf_index}");
}
