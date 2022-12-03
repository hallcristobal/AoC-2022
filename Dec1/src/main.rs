use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let mut buffer = BufReader::new(file);
    let mut file_input = String::new();
    buffer.read_to_string(&mut file_input)?;

    let mut elf_calories = parse_calories(&file_input)?;
    elf_calories.sort_by(|a, b| b.1.cmp(&a.1));

    if let Some(&(elf, calories)) = elf_with_highest_calories(&elf_calories) {
        println!("Highest Elf: {} - Calories: {}", elf, calories);
    } else {
        panic!("No elves in list?");
    }

    println!(
        "Total calories of top 3: {}",
        top_three_elves(&elf_calories)
    );

    Ok(())
}

// Part 1
fn elf_with_highest_calories(elf_calories: &Vec<(usize, usize)>) -> Option<&(usize, usize)> {
    elf_calories.first()
}

// Part 2
fn top_three_elves(elf_calories: &Vec<(usize, usize)>) -> usize {
    elf_calories
        .iter()
        .take(3)
        .fold(0, |acc, &(_, calories)| acc + calories)
}

fn parse_calories(input: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    let mut calories_map: Vec<(usize, usize)> = Vec::new();
    let mut current_elf = 0;
    let mut cumulative = 0;
    let mut counting = false;
    input.lines().into_iter().for_each(|line| {
        if line.is_empty() {
            calories_map.push((current_elf, cumulative));
            current_elf += 1;
            cumulative = 0;
            counting = false;
            return;
        }

        let cals = if let Ok(c) = line.parse::<usize>() {
            c
        } else {
            return;
        };

        counting = true;
        cumulative += cals;
    });

    if counting {
        calories_map.push((current_elf, cumulative));
    }

    Ok(calories_map)
}
