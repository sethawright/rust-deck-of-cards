use std::fmt;

#[derive(Debug)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rank::Ace => f.write_str("A"),
            Rank::Jack => f.write_str("J"),
            Rank::Queen => f.write_str("Q"),
            Rank::King => f.write_str("K"),
            // Weird side effect of having a borrowed value here

            // This is one of the easier ways to write this, but what's important to keep in mind is
            // there's a heap allocation here. We're allocating a new string every time we call this
            // function. So this can be a very expensive operation.
            // Rank::Number(num) => f.write_str(&num.to_string()),

            // If we want to avoid the heap allocation, we can use the unsafe version of from_utf8_unchecked
            // This is safe because we know the data is valid UTF-8
            Rank::Number(num) => {
                let first = num / 10;
                let second = num % 10;
                let mut buf = [0; 2];
                if first > 0 {
                    buf[0] = first + b'0';
                    buf[1] = second + b'0';
                } else {
                    buf[0] = second + b'0';
                }
                f.write_str(unsafe { std::str::from_utf8_unchecked(&buf) })
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

// This will allow us to print the suit
// It'll also work with #[derive(Debug)]
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Suit::Spade => "♠",
            Suit::Club => "♣",
            Suit::Heart => "♥",
            Suit::Diamond => "♦",
        })
    }
}

#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
