use std::collections::HashMap;

struct Hand {
    // sorted by Rank ascending
    cards: Vec<Card>
}

impl Hand {
    fn from(hand_str: &str) -> Hand {
        let mut cards: Vec<_> = hand_str.split(" ").map(Card::from).collect();
        if cards.len() != 5 {
            panic!("Invalid number of cards in hand {}", hand_str);
        }
        cards.sort_by_key(|card| card.rank);
        Hand { cards }
    }

    fn is_straight(&self) -> Option<Rank> {
        if has_ascending_rank(&self.cards) {
            return Some(self.cards[4].rank);
        } else if self.cards[4].rank == RANK_ACE && self.cards[0].rank == Rank(2) && has_ascending_rank(&self.cards[0..4]) {
            return Some(Rank(5));
        } else {
            None
        }
    }

    fn category(&self) -> HandCategory {
        let is_straight = self.is_straight();
        let all_have_same_suit = self.cards.iter().all(|c| c.suit == self.cards[0].suit);

        if let Some(straight_rank) = is_straight {
            if all_have_same_suit {
                return HandCategory::StraightFlush(straight_rank);
            }
        }

        let sorted_rank_groups = to_sorted_rank_groups(&self.cards);

        if sorted_rank_groups[0].1 == 4 {
            return HandCategory::FourOfAKind(sorted_rank_groups[0].0, sorted_rank_groups[1].0);
        }

        if sorted_rank_groups[0].1 == 3 && sorted_rank_groups[1].1 == 2 {
            return HandCategory::FullHouse(sorted_rank_groups[0].0, sorted_rank_groups[1].0);
        }

        if all_have_same_suit {
            return HandCategory::Flush([self.cards[4].rank, self.cards[3].rank,
                self.cards[2].rank, self.cards[1].rank, self.cards[0].rank]);
        }

        if let Some(straight_rank) = is_straight {
            return HandCategory::Straight(straight_rank);
        }


        if sorted_rank_groups[0].1 == 3 {
            return HandCategory::ThreeOfAKind([sorted_rank_groups[0].0, sorted_rank_groups[1].0, sorted_rank_groups[2].0]);
        }

        if sorted_rank_groups[0].1 == 2 && sorted_rank_groups[1].1 == 2 {
            return HandCategory::TwoPair([sorted_rank_groups[0].0, sorted_rank_groups[1].0, sorted_rank_groups[2].0]);
        }

        if sorted_rank_groups[0].1 == 2 {
            return HandCategory::OnePair([sorted_rank_groups[0].0, sorted_rank_groups[1].0, sorted_rank_groups[2].0, sorted_rank_groups[3].0]);
        }

        HandCategory::HighCard([self.cards[4].rank, self.cards[3].rank,
            self.cards[2].rank, self.cards[1].rank, self.cards[0].rank])
    }
}

fn has_ascending_rank(cards: &[Card]) -> bool {
    cards.windows(2).all(|pair| pair[0].rank.value() + 1 == pair[1].rank.value())
}

/// Groups are sorted by size (desc) and rank(desc)
/// Example [(Rank 2, 3),(Rank 10, 1),(Rank 9, 1)]
fn to_sorted_rank_groups(cards: &[Card]) -> Vec<(Rank, usize)> {
    let mut rank_and_count: Vec<_> = cards.iter()
        .fold(HashMap::new(), |mut map, card| {
            *map.entry(card.rank).or_insert(0) += 1;
            map
        }).into_iter().collect();
    rank_and_count.sort_by_key(|c| c.0);
    rank_and_count.sort_by_key(|c| c.1);
    rank_and_count.reverse();
    rank_and_count
}


#[derive(Copy, Clone, Debug)]
struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    fn from(card_str: &str) -> Card {
        let (head, tail) = card_str.split_at(card_str.len() - 1);
        Card {
            rank: Rank::from(head),
            suit: Suit::from(tail.chars().next().unwrap())
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Suit {
    CLUB,
    DIAMOND,
    HEART,
    SPADE
}

use Suit::*;

impl Suit {
    fn from(suit_char: char) -> Suit {
        match suit_char {
            'C' => CLUB,
            'D' => DIAMOND,
            'H' => HEART,
            'S' => SPADE,
            _ => panic!("Invalid suit char {}", suit_char)
        }
    }
}


#[derive(Copy, Clone, Ord, Eq, PartialEq, PartialOrd, Hash, Debug)]
pub struct Rank(u8);

const RANK_ACE: Rank = Rank(14);

impl Rank {
    pub fn from(rank_str: &str) -> Rank {
        match rank_str {
            "A" => RANK_ACE,
            "K" => Rank(13),
            "Q" => Rank(12),
            "J" => Rank(11),
            n_str => {
                let n = n_str.parse().unwrap();
                if n < 2 || n > 10 {
                    panic!("Invalid rank {}", rank_str)
                }
                Rank(n)
            }
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}


#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum HandCategory {
    HighCard([Rank; 5]),
    OnePair([Rank; 4]),
    TwoPair([Rank; 3]),
    ThreeOfAKind([Rank; 3]),
    Straight(Rank),
    Flush([Rank; 5]),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
}

pub fn category_of_hand_str(hand: &str) -> HandCategory {
    Hand::from(hand).category()
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hand_strs: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut hand_strs_with_category: Vec<_> = hand_strs.iter().map(|hand_str| (hand_str, category_of_hand_str(hand_str))).collect();
    hand_strs_with_category.sort_by(|&(_, ref category1), &(_, ref category2)| category2.cmp(category1));

    let winner_category = hand_strs_with_category[0].1.clone();
    let winners: Vec<&'a str> = hand_strs_with_category.into_iter()
        .take_while((|&(_, ref category)| *category == winner_category))
        .map(|(&hand_str, _)| hand_str)
        .collect();

    Some(winners)
}



