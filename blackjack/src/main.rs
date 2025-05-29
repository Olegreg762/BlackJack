use std::collections::HashMap;
use std::io;

fn main() {
    let mut money: i32 = 100;
    
    loop{
        play_game(&mut money);
        if !play_again(){
            break;
        }
    }
}
fn player_money(money: &mut i32, change: i32) -> i32 {
    if change > 0 {
        *money += change * 2;
    } else {
        *money += change;
    }
    if *money <= 0 {
        println!("You have no money left! Game over.");
        std::process::exit(0);
    }
    println!("You currently have ${}.", money);
    *money
    
}

fn bet(money: i32) -> i32 {
    loop {
        let mut bet_amount = String::new();
        println!("You have ${}. How much would you like to bet?", money);

        io::stdin()
            .read_line(&mut bet_amount)
            .expect("Failed to read line");

        match bet_amount.trim().parse::<i32>() {
            Ok(num) if num > 0 && num <= money => {
                println!("You have bet ${}.", num);
                return num;
            },
            Ok(_) => println!("Invalid bet amount. You must bet between $1 and ${}.", money),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn play_game(money: &mut i32) {
    player_money(money, 0);
    let bet_amount: i32 = bet(*money);
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
        let choice= choice.trim().to_lowercase();

        match choice.as_str() {
            "h" | "hit" => {
                deal_card(&mut deck, &mut player_hand);
                see_hand(&mut player_hand, "Player");
                if has_bust(&mut player_hand, "Player") {
                    player_money(money, -bet_amount);
                    return;
                    
                }
                if has_blackjack(&mut player_hand, "Player") {
                    player_money(money, bet_amount);
                    return;
                }
            },
            "s" | "stay" => {
                println!("You chose to stay.");
                break;
            },
            _ => println!("Invalid choice. Please enter 'h', 'hit', 's', or 'stay'."),
        }
    }
    while check_hand_value(&mut dealer_hand, "Dealer") < 17 {
        deal_card(&mut deck, &mut dealer_hand);
        if has_bust(&mut dealer_hand, "Dealer"){
            player_money(money, bet_amount);
            return;
        }
        if has_blackjack(&mut dealer_hand, "Dealer") {
            player_money(money, -bet_amount);
            return;
        }
    }
    see_hand(&mut dealer_hand, "Dealer");
    if check_hand_value(&mut dealer_hand, "Dealer") > check_hand_value(&mut player_hand, "Player") {
        println!("Dealer wins!, you lost ${}.", bet_amount);
        player_money(money, -bet_amount);
    } else if check_hand_value(&mut dealer_hand, "Dealer") < check_hand_value(&mut player_hand, "Player") {
        println!("Player wins!, you won ${}.", bet_amount * 2);
        player_money(money, bet_amount);
    } else {
        println!("It's a tie!");
    }
    
}
fn has_blackjack(hand: &mut HashMap<String, i8>, hand_name: &str) -> bool {
    let value: i8 = check_hand_value(hand, hand_name);
    if value == 21 {
        println!("{} has BlackJack!!!\n", hand_name);
        println!("{} has won the game.\n", hand_name);
        return true;
    }
    false
}
fn has_bust(hand: &mut HashMap<String, i8>, hand_name: &str) -> bool{
    let value: i8 = check_hand_value(hand, hand_name);
    if value > 21 {
        println!("{} has busted!\n", hand_name);
        println!("{} has lost the game.\n", hand_name);
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
