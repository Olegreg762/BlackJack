use std::collections::HashMap;

fn main() {
    let mut deck: HashMap<String, i8> = build_deck();
    let mut player_hand: HashMap<String, i8> = HashMap::new();
    let mut dealer_hand: HashMap<String, i8> = HashMap::new();

    for _ in 0..2 {
        deal_card(&mut deck, &mut player_hand);
        deal_card(&mut deck, &mut dealer_hand);
    }

    see_hand(&player_hand, "Player");
    see_hand(&dealer_hand, "Dealer");

}

fn see_hand(hand: &HashMap<String, i8>, hand_name: &str) {
    println!("Cards in {} Hand:", hand_name);
    for (card, value) in hand.iter() {
        println!("{}: {}", card, value);
    }
    let total: i8 = hand.values().sum();
    println!("{}'s Hand value: {}\n", hand_name, total);
}

fn build_deck() -> HashMap<String, i8> {
    let mut deck: HashMap<String, i8> = HashMap::new();
    let suits: [&'static str; 4]= ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks: [&'static str; 13]= ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];
    let values: [i8; 13]= [2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11];

    for suit in suits.iter() {
        for (i, rank) in ranks.iter().enumerate(){
            let card: String = format!("{} of {}", rank, suit);
            deck.insert(card, values[i]);
        }
    }
    deck
}

fn deal_card(deck: &mut HashMap<String, i8>, hand: &mut HashMap<String, i8>) {
    if let Some((card, &value)) = deck.iter().next() {
        let card_name = card.clone();
        deck.remove(&card_name);
        hand.insert(card_name, value);
    }
}
    

