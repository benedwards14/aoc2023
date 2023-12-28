use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cards {
    rank: i32,
    card_1: i32,
    card_2: i32,
    card_3: i32,
    card_4: i32,
    card_5: i32
}

#[derive(Debug)]
struct Hand {
    cards: Cards,
    bid: i32
}

const FIVE_OF_A_KIND: i32 =7;
const FOUR_OF_A_KIND: i32 =6;
const FULL_HOUSE: i32 = 5;
const THREE_OF_A_KIND: i32 =4; 
const TWO_PAIR: i32 = 3; 
const ONE_PAIR: i32 = 2;
const HIGH_CARD: i32 = 1;

impl Cards {
    fn parse(cards: &str) -> Self {
        let mut card_count = HashMap::new();
        let mut parsed_cards = Vec::new();
        let mut joker_count = 0;

        for card in cards.trim().chars() {
            if card == 'J' {
                joker_count += 1;
            } else {
                card_count.entry(card).and_modify(|cc| *cc += 1).or_insert(1);
            }
            let parsed_card: i32 = match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                c => c.to_digit(10).unwrap()
            }.try_into().unwrap();

            parsed_cards.push(parsed_card);
        }

        let max_card_count = *card_count.values().into_iter().max().unwrap_or(&0);
        let rank = match max_card_count {
            5 => {
                assert!(joker_count == 0);
                FIVE_OF_A_KIND
            },
            4 => {
                match joker_count {
                    0 => FOUR_OF_A_KIND,
                    1 => FIVE_OF_A_KIND,
                    _ => panic!("4")
                }
            },
            3 if card_count.len() == 2 => {
                match joker_count {
                    0 => FULL_HOUSE,
                    1 => FOUR_OF_A_KIND,
                    _ => panic!("full house")
                    
                }
            },
            3 => {
                match joker_count {
                    0 => THREE_OF_A_KIND,
                    1 => FOUR_OF_A_KIND,
                    2 => FIVE_OF_A_KIND,
                    _ => panic!("3")
                }
            },
            2 if card_count.len() == 3 => {
                match joker_count {
                    0 => TWO_PAIR,
                    1 => THREE_OF_A_KIND,
                    _ => panic!("two pair")
                }
            },
            2 if card_count.len() == 2  => {
                match joker_count {
                    1 => FULL_HOUSE,
                    2 => FOUR_OF_A_KIND,
                    _ => panic!("two pair 2")
                    
                }
            }
            2 => {
                match joker_count {
                    0 => ONE_PAIR,
                    1 => THREE_OF_A_KIND,
                    2 => FOUR_OF_A_KIND,
                    3 => FIVE_OF_A_KIND,
                    _ => panic!("2")
                }
            },
            1 => {
                match joker_count {
                    0 => HIGH_CARD,
                    1 => ONE_PAIR,
                    2 => THREE_OF_A_KIND,
                    3 => FOUR_OF_A_KIND,
                    4 => FIVE_OF_A_KIND,
                    _ => panic!("1")
                }
            },
            _ => {
                assert!(joker_count == 5);
                FIVE_OF_A_KIND
            }
        };

        Cards {
            rank,
            card_1: parsed_cards[0],
            card_2: parsed_cards[1],
            card_3: parsed_cards[2],
            card_4: parsed_cards[3],
            card_5: parsed_cards[4]
        }
    }
}


fn parse_input() -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in read_to_string("./data.txt").unwrap().lines() {
        let (cards, bid) = line.split_once(" ").unwrap() ;
        hands.push(Hand { cards: Cards::parse(cards), bid: bid.parse().unwrap() });
    }

    hands
}

fn main() {
    let mut input = parse_input();

    input.sort_by(|k1, k2| k1.cards.cmp(&k2.cards));

    let mut sum = 0;
    for (idx, hand) in input.iter().enumerate() {
        let rank: i32 = (idx + 1).try_into().unwrap();
        sum += rank * hand.bid;
    }
    println!("{}", sum); // 251927063 255632664

}
