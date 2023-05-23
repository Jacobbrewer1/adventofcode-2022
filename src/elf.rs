// Elf is a struct that represents a person.
#[derive(Clone)]
pub struct Elf {
    // Id is the id of the elf.
    pub id: i32,
    // Calories is the number of calories the elf has got on them.
    pub calories: i32,
}

impl Elf {
    // Create a new elf. The elf starts with 0 calories.
    pub fn new() -> Elf {
        Elf {
            id: 0,
            calories: 0,
        }
    }

    // Add calories to the elf.
    pub fn add_calories(&mut self, calories: i32) {
        self.calories += calories;
    }

    // Get the number of calories the elf has.
    pub fn get_calories(&self) -> i32 {
        self.calories
    }
}
