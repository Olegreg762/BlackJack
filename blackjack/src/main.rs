use std::collections::HashMap;
use std::io;

fn main() {
    loop{
    play_game();
    if !play_again(){
        break;
    }
    }
}

fn play_game() {
    let mut deck: HashMap<String, i8> = build_deck();
    let mut player_hand: HashMap<String, i8> = HashMap::new();
    let mut dealer_hand: HashMap<String, i8> = HashMap::new();

    for _ in 0..2 {
        deal_card(&mut deck, &mut player_hand);
        deal_card(&mut deck, &mut dealer_hand);
    }
    
    if has_bust(&mut player_hand, "Player") || has_bust(&mut dealer_hand, "Dealer"){
        return;
    }

    see_hand(&mut player_hand, "Player");

    loop {
        let mut choice: String = String::new();
        println!("Do you want to hit or stay? (h/s)");
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: char = choice.trim().chars().next().unwrap_or(' ');

        match choice {
            'h' => {
                deal_card(&mut deck, &mut player_hand);
                see_hand(&mut player_hand, "Player");
                if has_bust(&mut player_hand, "Player") {
                    return;
                }
            },
            's' => {
                println!("You chose to stay.");
                break;
            },
            _ => println!("Invalid choice. Please enter 'h' or 's'."),
        }
    }
    while check_hand_value(&mut dealer_hand, "Dealer") < 17 {
        deal_card(&mut deck, &mut dealer_hand);
        if has_bust(&mut dealer_hand, "Dealer"){
            return;
        }
    }
    
    if check_hand_value(&mut dealer_hand, "Dealer") > check_hand_value(&mut player_hand, "Player") {
        println!("Dealer wins!");
    } else if check_hand_value(&mut dealer_hand, "Dealer") < check_hand_value(&mut player_hand, "Player") {
        println!("Player wins!");
    } else {
        println!("It's a tie!");
    }
    
}

fn has_bust(hand: &mut HashMap<String, i8>, hand_name: &str) -> bool{
    let value: i8 = check_hand_value(hand, hand_name);
    if value > 21 {
        println!("{} has busted!\n", hand_name);
        println!("{} has lost the game.\n", hand_name);
        return true;
    } else if value == 21 {
        println!("{} has BackJack!!!\n", hand_name);
        println!("{} has won the game.\n", hand_name);
        return true;
    } else {
        println!("{} is still in the game.\n", hand_name);
        return false;
    }
}

fn check_hand_value(hand: &mut HashMap<String, i8>, _hand_name: &str) -> i8 {
    let mut total: i8 = hand.values().sum();
    if total > 21 && hand.keys().any(|card| card.starts_with("Ace")) {
        for (card, value) in hand.iter_mut() {
            if card.starts_with("Ace") && *value == 11 {
                *value = 1;
                total = hand.values().sum();
                break;
            }
        }
    }
    total
}

fn see_hand(hand: &mut HashMap<String, i8>, hand_name: &str) {
    println!("Cards in {} Hand:", hand_name);
    for (card, value) in hand.iter() {
        println!("{}: {}", card, value);
    }
    println!("Hand Value: {}",check_hand_value(hand, hand_name));
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
    
fn play_again() -> bool{
    loop{
    let mut play_again: String = String::new();
    println!("Do you want to play again? (y/n)");
    io::stdin().read_line(&mut play_again).expect("Failed to read line");
    let play_again: char = play_again.trim().chars().next().unwrap_or(' ');
    
    match play_again {
        'y' => {
            println!("Starting a new game...");
            return true;    
        },
        'n' => {
            println!("Thanks for playing!");
            return false;           
        },
        _ => {
            println!("Invalid choice. Please enter 'y' or 'n'.");            
        }
        }
    }
}
