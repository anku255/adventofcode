use std::fs;

#[derive(Debug)]
struct Card {
    our_numbers: Vec<i32>,
    winning_numbers: Vec<i32>,
    wins: i32
}

fn part1(cards: &Vec<Card>) -> i32 {
    let mut total_points = 0;
    for card in cards {
        let mut points = 0;
        for our_number in &card.our_numbers {
            if card.winning_numbers.contains(&our_number) {
                points  = if points == 0 { 1 } else { points * 2 };
            }
        }
        total_points += points;
    }
    total_points
}

fn get_total_scratchcards(cards: &Vec<Card>, card_index: usize) -> i32 {
    let card = &cards[card_index];
    let mut total = card.wins;
    let end_index = card_index + card.wins as usize;

    for i in card_index..end_index {
        total += get_total_scratchcards(cards, i + 1)
    }

    return total;
}

fn part2(cards: &Vec<Card>) -> i32 {
    let mut total = cards.len() as i32;
    for i in 0..cards.len() {
        let card_index: usize = i.try_into().unwrap();
        total += get_total_scratchcards(cards, card_index);
    }

    total
}

fn main() {
    let filename = std::env::args().nth(1).expect("No input file specified! Usage: cargo run <input_file>");

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str>  = contents.split("\n").collect();

    let mut cards: Vec<Card> = Vec::new();

    for line in &lines {
        let card_str: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>().iter().map(|x| x.trim()).collect::<Vec<&str>>();

        let winning_numbers: Vec<i32> = card_str[0].split(" ").collect::<Vec<&str>>().iter().filter(|x| x.trim() != "").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let our_numbers: Vec<i32> = card_str[1].split(" ").collect::<Vec<&str>>().iter().filter(|x| x.trim() != "").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let matching_numbers: Vec<i32> = our_numbers.iter().filter(|x| winning_numbers.contains(x)).map(|x| *x).collect::<Vec<i32>>();

        cards.push(Card { our_numbers, winning_numbers, wins: matching_numbers.len() as i32 });
    }

    let total_points = part1(&cards);
    let total_scratchcards = part2(&cards);
    println!("Total points: {}", total_points);
    println!("Total Scratchcards: {}", total_scratchcards);
}