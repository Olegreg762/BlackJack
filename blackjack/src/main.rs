use std::collections::HashMap;

fn build_deck() -> HashMap<String, i8> {
    let mut deck: HashMap<String, i8> = HashMap::new();
    let suits= ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks= ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];
    let values= [2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11];

    for suit in suits.iter() {
        for (i, rank) in ranks.iter().enumerate(){
            let card = format!("{} of {}", rank, suit);
            deck.insert(card, values[i]);
        }
    }

    deck

}
fn main() {
    let deck = build_deck();
    println!("{:?}", deck);
}
