use std::collections::HashMap;

struct Round {
    bet: u32,
    score: u32,
    joker_score: u32,
}

enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    Nothing = 0,
}

pub fn run_day() {
    let mut rounds = parse_input();
    println!("Day 7 Part 1: Total winnings are {}", part_one(&mut rounds));
    println!(
        "Day 7 Part 1: Winnings with joker rule are {}",
        part_two(&mut rounds)
    );
    println!();
}

fn part_one(rounds: &mut Vec<Round>) -> u32 {
    let mut winnings = 0;

    // sort descending by score to get vec where index is rank minus one
    rounds.sort_by(|a, b| a.score.cmp(&b.score));

    for (index, round) in rounds.iter().enumerate() {
        let rank: u32 = (index + 1).try_into().unwrap();
        winnings += rank * round.bet;
    }

    winnings
}

fn part_two(rounds: &mut Vec<Round>) -> u32 {
    let mut winnings = 0;

    // sort descending by score (with joker rule) to get vec where index is rank minus one
    rounds.sort_by(|a, b| a.joker_score.cmp(&b.joker_score));

    for (index, round) in rounds.iter().enumerate() {
        let rank: u32 = (index + 1).try_into().unwrap();
        winnings += rank * round.bet;
    }

    winnings
}

fn parse_input() -> Vec<Round> {
    let input = include_str!("input.txt");
    let mut rounds = Vec::new();

    for line in input.lines() {
        let (hand, bet_str) = line.split_once(" ").unwrap();
        let bet: u32 = bet_str.parse().unwrap();
        let round = Round {
            bet: bet,
            score: compute_hand_score(hand, false),
            joker_score: compute_hand_score(hand, true),
        };
        rounds.push(round);
    }

    rounds
}

/// Score is:
/// 6 * 15⁵ for five of a kind, 5*15⁵ for four of a kind etc. down to 0 for all distinct
/// Plus: first card value * 15⁴, second card value * 15³ and so on to fifth card value * 1
/// If joker_rule is true, jokers are worth 1 individually (rather than 11) but counted as most beneficial other card for hand type
fn compute_hand_score(hand_str: &str, joker_rule: bool) -> u32 {
    let mut score = 0;
    let hand: Vec<u32> = hand_str   // vec of cards in hand as scores (rather than chars)
        .chars()
        .map(|card| get_card_score(card, joker_rule))
        .collect();
    let mut card_count: HashMap<u32, u32> = HashMap::new(); // maps card score to count of this card type in hand

    // count occurences of individual cards and add their positional score
    for (i, card_score) in hand.iter().enumerate() {
        // increment (and create if necessary) entry for this card type in hash table
        let entry = card_count.entry(*card_score).or_insert(0);
        *entry += 1;

        // calculate positional card score and add to total score
        let exponent: u32 = (4 - i).try_into().unwrap();
        let position_factor = 15_u32.pow(exponent);
        score += card_score * position_factor;
    }

    // count jokers for joker rule
    let joker_count = card_count
        .entry(get_card_score('J', joker_rule))
        .or_insert(0);
    let jokers = *joker_count;

    // if the joker rule is active, don't count jokers as cards later on (because they will become another card)
    if joker_rule {
        *joker_count = 0;
    }

    // get card counts out of hash map (type is now irrelevant) and sort descending
    let mut counts: Vec<u32> = card_count.values().cloned().collect();
    counts.sort_by(|a, b| b.cmp(a));

    // it's always most beneficial for a joker to count as the most common card
    if joker_rule {
        counts[0] += jokers;
    }

    // calculate score for hand type
    score += get_hand_type(counts) as u32 * 15_u32.pow(5);

    score
}

/// returns card score corresponding to card character
/// joker_rule will make joker worth 1 instead of 11
fn get_card_score(card: char, joker_rule: bool) -> u32 {
    match card {
        'A' => return 14,
        'K' => return 13,
        'Q' => return 12,
        'J' => {
            if joker_rule {
                return 1;
            } else {
                return 11;
            }
        }
        'T' => return 10,
        '9' => return 9,
        '8' => return 8,
        '7' => return 7,
        '6' => return 6,
        '5' => return 5,
        '4' => return 4,
        '3' => return 3,
        '2' => return 2,
        _ => return 0,
    }
}

/// returns hand type when given sorted vec (descending) containing card counts of a kind in hand
fn get_hand_type(counts: Vec<u32>) -> HandType {
    let hand_type;

    match counts[0] {
        5 => hand_type = HandType::FiveOfAKind,
        4 => hand_type = HandType::FourOfAKind,
        3 => {
            if counts[1] == 2 {
                hand_type = HandType::FullHouse;
            } else {
                hand_type = HandType::ThreeOfAKind;
            }
        }
        2 => {
            if counts[1] == 2 {
                hand_type = HandType::TwoPairs;
            } else {
                hand_type = HandType::OnePair;
            }
        }
        _ => hand_type = HandType::Nothing,
    }

    hand_type
}
