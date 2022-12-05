mod data;
use data::DAY3_DATA;

enum SupplyPriority {
    High,
    Low
}

/// Casting to a u8 isn't normally safe because a char might be one byte or four.
/// Because the code here is static, I'm considering this safe practice but don't
/// assume you can or should always do this
fn map_priority(input: u8) -> (SupplyPriority, u8) {
    if input >= 'A' as u8 && input <= 'Z' as u8 { 
        (SupplyPriority::Low, input + 27 - 'A' as u8)
    } else if input >= 'a' as u8 && input <= 'z' as u8 {
        (SupplyPriority::High, input + 1 - 'a' as u8)
    } else {
        panic!("Bad data. Found '{input}' instead of [a-zA-Z]");
    }
}

pub fn main() {
    let mut total_priorities: u32 = 0;

    for (rucksack_index, line) in DAY3_DATA.split('\n').enumerate() {
        // Compartments holding each half of the items
        let (compartment1, compartment2) = line.split_at(line.len() / 2);
        
        // List containing one each of the items found to be duplicated
        let mut duplicate_types = Vec::new();

        // Make sure our number of items is even
        assert!(compartment1.len() == compartment2.len());

        // If we find a match in the second compartment of an item from the first
        // store a single copy in duplicate_types
        for item in compartment1.chars() {
            if compartment2.contains(item) {
                if !duplicate_types.contains(&item) {
                    duplicate_types.push(item);
                }
            }
        }

        for item in duplicate_types {
            // We can only safely cast a u8 here if we're in ASCII land
            assert!(item.len_utf8() == 1);
            let (_, current_value) = map_priority(item as u8);

            println!("Rucksack {rucksack_index:02}: Duplicate item types(s) {current_value} ({item})");

            total_priorities += current_value as u32;
        }
    }

    println!();
    println!("Total value of all duplicates is {total_priorities}");
}