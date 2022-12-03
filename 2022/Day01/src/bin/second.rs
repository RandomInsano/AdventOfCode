#![allow(non_snake_case)]

mod data;
use data::DATA;

fn insert_highest(list: &mut Vec<u32>, new_value: u32, limit: usize) {
    let mut insert_at = list.len();

    for (index, value) in list.iter().enumerate() {
        if new_value >= *value {
            insert_at = index;
            break;
        }
    }

    list.insert(insert_at, new_value);

    while list.len() > limit {
        let _ = list.pop();
    }
}

fn main() {
    let mut current_calories = 0;
    let mut richest_elf_list = Vec::new();
    let total_richest_calories: u32;
    const TOP_LIMIT: usize = 3;

    for line in DATA.split('\n') {
        match line.parse::<u32>() {
            Ok(x) => {
                current_calories += x;
            },
            Err(_) => {
                insert_highest(&mut richest_elf_list, current_calories, TOP_LIMIT);

                current_calories = 0;
            }
        }
    }

    total_richest_calories = richest_elf_list.iter().sum();

    println!("Top three richest elves have {total_richest_calories} calories between them");
    println!();
    println!("Richest elves calories are {richest_elf_list:#?}");
}

#[cfg(test)]
mod tests {
    use crate::insert_highest;

    #[test]
    fn test_insert_highest() {
        let mut numbers = Vec::new();

        for number in [1, 2, 5, 4, 3, 5, 5] {
            insert_highest(&mut numbers, number, 3);
        }

        print!("List: {:#?}", numbers);

        let answer = Vec::from([5, 5, 5]);
        assert!(numbers.eq(&answer));
    }
}
