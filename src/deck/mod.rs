mod card;

use card::{Card, Rank, Suit};
use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    // shuffle with the fisher-yates algorithm
    pub fn shuffle(&mut self) -> () {
        for i in 0..self.cards.len() {
            let index = self.random_index();
            self.cards.swap(i, index);
        }
    }

    pub fn pull_random_card(&mut self) -> Card {
        self.cards.remove(self.random_index())
    }

    pub fn pull_top_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn random_index(&self) -> usize {
        rand::thread_rng().gen_range(0..self.cards.len())
    }

    pub fn build() -> Self {
        // We know the size up front so we can pre-allocate
        let mut cards: Vec<Card> = Vec::with_capacity(52);
        // We can own the slice here because we're copying the values
        for suit in [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond] {
            for i in 2..=14 {
                match i {
                    2..=10 => cards.push(Card {
                        // We shouldn't need to clone here.
                        // Copying the value is faster
                        suit,
                        rank: Rank::Number(i),
                    }),
                    11 => cards.push(Card {
                        suit,
                        rank: Rank::Jack,
                    }),
                    12 => cards.push(Card {
                        suit,
                        rank: Rank::Queen,
                    }),
                    13 => cards.push(Card {
                        suit,
                        rank: Rank::King,
                    }),
                    14 => cards.push(Card {
                        suit,
                        rank: Rank::Ace,
                    }),
                    _ => unreachable!("Invalid card"),
                }
            }
        }

        Self { cards }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut deck = String::from("");
        deck.push_str(format!("{} cards: [  ", self.cards.len()).as_str());
        self.cards
            .iter()
            .for_each(|card| deck.push_str(format!("{}  ", card).as_str()));
        deck.push_str("]");
        write!(f, "{}", deck)
    }
}
