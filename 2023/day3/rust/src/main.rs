use std::fs;

#[derive(Debug)]
struct GridItem {
    x: i32,
    y: i32,
    value: String
}

#[derive(Debug)]
struct Gear {
    gear_ratio: i32
}

#[derive(Debug)]
struct Boundary {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32
}

fn parse_grid(lines: &Vec<&str>) -> (Vec<GridItem>, Vec<GridItem>) {
    let mut nums: Vec<GridItem> = Vec::new();
    let mut symbols: Vec<GridItem> = Vec::new();

    let mut num_x_index = usize::MAX;
    let mut num = String::from("");

    for (y, line) in lines.iter().enumerate() {
        for (x, val) in line.chars().enumerate() {
            // if char is a number
            if val.is_digit(10) {
                num += &val.to_string();
                num_x_index = if num_x_index == usize::MAX { x } else { num_x_index };
                if x < line.len() - 1 {
                    continue;
                }
            } else if val == '.' {
                //  do nothing 
            } else {
                symbols.push(GridItem {
                    x: x as i32,
                    y: y as i32,
                    value: val.to_string()
                })
            }

            if num_x_index != usize::MAX {
                nums.push(GridItem {
                    x: num_x_index as i32,
                    y: y as i32,
                    value: num
                });
                num = String::from("");
                num_x_index = usize::MAX;
            }
        }
    }

    (nums, symbols)

}

fn find_parts_and_gears(lines: &Vec<&str>) -> (Vec<GridItem>, Vec<Gear>) {
    let mut parts: Vec<GridItem> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();

    let (nums, symbols) = parse_grid(lines);

    for symbol in &symbols {
        let mut adjacent_parts_count = 0;
        let mut gear_ratio = 1;
        for num in &nums {
            let boundary = Boundary {
                min_x: num.x - 1,
                min_y: num.y - 1,
                max_x: num.x + num.value.len() as i32,
                max_y: num.y + 1
            };

            let is_symbol_within_boundary = 
              symbol.x >= boundary.min_x &&
              symbol.y >= boundary.min_y &&
              symbol.x <= boundary.max_x && 
              symbol.y <= boundary.max_y;

            if is_symbol_within_boundary {
                parts.push(GridItem {
                    x: num.x,
                    y: num.y,
                    value: num.value.clone()
                });

                adjacent_parts_count += 1;
                gear_ratio *= num.value.parse::<i32>().unwrap()
            }
        }

        if adjacent_parts_count == 2 {
            gears.push(Gear { gear_ratio })
        }
    }
    
    (parts, gears)

}

fn part1(lines: &Vec<&str>) -> i32 {
    let (parts, _) = find_parts_and_gears(lines);

    let result = parts.iter().fold(0, |acc, part| {
        acc + part.value.parse::<i32>().unwrap()
    });

    return result;
}

fn part2(lines: &Vec<&str>) -> i32 {
    let (_, gears) = find_parts_and_gears(&lines);

    let result = gears.iter().fold(0, |acc, gear| {
        acc + gear.gear_ratio
    });

    return result;
}

fn main() {
    let filename = std::env::args().nth(1).expect("No input file specified! Usage: cargo run <input_file>");
    let contents: String = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines:  Vec<&str> = contents.split("\n").collect();

    let result1 = part1(&lines);
    let result2: i32 = part2(&lines);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}
