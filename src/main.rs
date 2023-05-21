mod deck;
use deck::Deck;

fn main() {
    let mut deck = Deck::build();
    deck.shuffle();

    let random_card = deck.pull_random_card();
    println!("Your random card is: {}", random_card);

    if let Some(top_card) = deck.pull_top_card() {
        println!("Top card is: {}", top_card);
    } else {
        println!("No cards in deck.");
    }

    println!("{}", deck);
}
