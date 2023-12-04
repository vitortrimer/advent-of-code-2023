use std::{fs, i32};

fn main() {
    // DAY 01
    day_one_first_part();
    day_one_second_part();
}

fn day_one_first_part() {
    let puzzle_input =
        fs::read_to_string("./puzzle_input.txt").expect("Should have been able to read the file");

    let items: Vec<&str> = puzzle_input.lines().collect();
    let mut decoded_items: Vec<i32> = vec![];

    for item in items {
        let numbers = item.chars().filter(|n| n.is_numeric()).collect::<Vec<_>>();

        if numbers.first().is_some() {
            let first_number = numbers.first().unwrap();
            let last_number = numbers.last().unwrap();
            let result_item = format!("{first_number}{last_number}").parse::<i32>();

            match result_item {
                Ok(result) => {
                    decoded_items.push(result);
                }
                Err(_) => {}
            }
        }
    }

    let mut sum_result: i32 = 0;
    for item in decoded_items {
        sum_result += item;
    }

    print!("Final result is {sum_result}");
}

fn day_one_second_part() {
    let puzzle_input =
        fs::read_to_string("./puzzle_input.txt").expect("Should have been able to read the file");

    let items: Vec<&str> = puzzle_input.lines().collect();
    let mut decoded_items: Vec<i32> = vec![];

    for item in items {
        let normalized_numbers_string = string_to_number(item.to_string());
        let numbers = normalized_numbers_string
            .chars()
            .filter(|n| n.is_numeric())
            .collect::<Vec<_>>();

        if numbers.first().is_some() {
            let first_number = numbers.first().unwrap();
            let last_number = numbers.last().unwrap();

            let result_item = format!("{first_number}{last_number}").parse::<i32>();
            match result_item {
                Ok(result) => {
                    decoded_items.push(result);
                }
                Err(_) => {
                    println!("failed one item");
                }
            }
        }
    }

    let mut sum_result: i32 = 0;
    for item in decoded_items {
        sum_result += item;
    }

    print!("Final result is {sum_result}");
}

fn string_to_number(line: String) -> String {
    let mut parsed_result: String = line;

    parsed_result = parsed_result.replace("oneight", "18");
    parsed_result = parsed_result.replace("twone", "21");
    parsed_result = parsed_result.replace("threeight", "38");
    parsed_result = parsed_result.replace("fiveight", "58");
    parsed_result = parsed_result.replace("eightwo", "82");
    parsed_result = parsed_result.replace("eighthree", "83");
    parsed_result = parsed_result.replace("nineight", "98");
    parsed_result = parsed_result.replace("one", "1");
    parsed_result = parsed_result.replace("two", "2");
    parsed_result = parsed_result.replace("three", "3");
    parsed_result = parsed_result.replace("four", "4");
    parsed_result = parsed_result.replace("five", "5");
    parsed_result = parsed_result.replace("six", " 6");
    parsed_result = parsed_result.replace("seven", "7");
    parsed_result = parsed_result.replace("eight", "8");
    parsed_result = parsed_result.replace("nine", "9");

    parsed_result
}
