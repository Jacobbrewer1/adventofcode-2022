use std::fs;
use crate::elf;

pub fn run() {
    // Read contents of file
    let contents = fs::read_to_string("./assets/day1_input.txt")
        .expect("Something went wrong reading the file");

    let elfs: Vec<elf::Elf> = process_file(contents);

    // Copy the elfs vector
    let elfs_copy = elfs.clone();

    // Find the elf with the most calories
    let mut max_calories = 0;
    let mut max_calories_elf = elf::Elf::new();
    for elf in elfs {
        if elf.get_calories() > max_calories {
            max_calories = elf.get_calories();
            max_calories_elf = elf;
        }
    }

    // Print the elf with the most calories
    println!("Elf {} has the most calories with {} calories", max_calories_elf.id, max_calories_elf.calories);

    // Find top 3 elfs with the most calories
    let mut top_elfs: Vec<elf::Elf> = Vec::new();
    for elf in elfs_copy {
        if top_elfs.len() < 3 {
            top_elfs.push(elf);
            continue;
        }
        // Sort the top elfs by calories
        top_elfs.sort_by(|a, b| b.calories.cmp(&a.calories));
        // If the current elf has more calories than the lowest elf, replace the lowest elf
        if elf.calories > top_elfs[2].calories {
            top_elfs[2] = elf;
        }
    }

    // Print the top 3 elfs
    println!("The top 3 elfs are:");
    for elf in &top_elfs {
        println!("Elf {} with {} calories", elf.id, elf.calories);
    }

    // Find the total of the top 3 elfs
    let mut total = 0;
    for elf in &top_elfs {
        total += elf.calories;
    }

    // Print the total
    println!("The total of the top 3 elfs is {}", total);
}

// process the file
fn process_file(contents: String) -> Vec<elf::Elf> {
    // Create a new Vec of elfs
    let mut elfs: Vec<elf::Elf> = Vec::new();

    // count the number of elfs
    let mut elf_count = 0;

    // Create the beginning elf
    let mut elf = elf::Elf::new();

    // loop through each line of the file
    for line in contents.lines() {
        // If line is empty, complete elf by adding it to the end of the vector
        if line.is_empty() {
            // Assign the elf the next id
            elf.id = elf_count;
            // Increment the elf count
            elf_count += 1;
            // Add the elf to the vector
            elfs.push(elf);
            // Create a new elf
            elf = elf::Elf::new();
            continue;
        }
        // Get the line contents as a number
        let number: i32 = line.parse().unwrap();
        // Add the number to the elf
        elf.add_calories(number);
    }
    return elfs;
}