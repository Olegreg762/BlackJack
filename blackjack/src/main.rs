use std::collections::HashMap;
use std::io;


struct Player {
    name: String,
    hand: HashMap<String, i8>,
    money: i32
}

impl Player {
    fn new(name: &str, starting_money: i32) -> Self {
        Self {
            name: name.to_string(),
            hand: HashMap::new(),
            money: starting_money,
        }
    }

    fn check_hand_value(&mut self) -> i8 {
        let mut total: i8 = self.hand.values().sum();

        if total > 21 && self.hand.keys().any(|card| card.starts_with("Ace")) {
            for (card, value) in self.hand.iter_mut() {
                if card.starts_with("Ace") && *value == 11 {
                    *value = 1;
                    total = self.hand.values().sum();
                    break;
                }
            }
        }
        total
    }

    fn see_hand(&mut self) {
        println!("Cards in {} Hand:", self.name);
        for (card, value) in &self.hand {
            println!("{}: {}", card, value);
        }
        println!("Hand Value: {}",self.check_hand_value());
    }

    fn has_blackjack(&mut self) -> bool {
        let value: i8 = self.check_hand_value();
        if value == 21 {
            println!("{} has BlackJack!!!\n", &self.name);
            println!("{} has won the game.\n", &self.name);
            return true;
        }
        false
    }

    fn has_bust(&mut self) -> bool {
        let value: i8 = self.check_hand_value();
        if value > 21 {
            println!("{} has busted!\n", &self.name);
            println!("{} has lost the game.\n", &self.name);
            return true;
        } else {
            println!("{} is still in the game.\n", &self.name);
            return false;
        }
    }

    fn bet(&self) -> i32 {
        loop {
            let mut bet_amount = String::new();
            println!("You have ${}. How much would you like to bet?", self.money);

            io::stdin()
                .read_line(&mut bet_amount)
                .expect("Failed to read line");

            match bet_amount.trim().parse::<i32>() {
                Ok(num) if num > 0 && num <= self.money => {
                    println!("You have bet ${}.", num);
                    return num;
                },
                Ok(_) => println!("Invalid bet amount. You must bet between $1 and ${}.", self.money),
                Err(_) => println!("Please enter a valid number."),
            }
        }
    }

    fn player_money(&mut self, change: i32) -> i32 {
        if change > 0 {
            self.money += change * 2;
        } else {
            self.money += change;
        }
        if self.money <= 0 {
            println!("You have no money left! Game over.");
            std::process::exit(0);
        }
        println!("You currently have ${}.", self.money);
        self.money
        
    }

    fn clear_hand(&mut self) {
        self.hand.clear();
    }
}

struct Deck {
    cards: HashMap<String, i8>,
}

impl Deck {
    fn new() -> Self {
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
        Self { cards: deck }
    }

    fn deal_card(&mut self, hand: &mut HashMap<String, i8>) {
        if let Some((card, &value)) = self.cards.iter().next() {
            let card_name = card.clone();
            self.cards.remove(&card_name);
            hand.insert(card_name, value);
        }
    }
}

struct Game {
    player: Player,
    dealer: Player,
    deck: Deck,
}

impl Game {
    fn new() -> Self {
        let player = Player::new("Player", 100);
        let dealer = Player::new("Dealer", 0);
        let deck = Deck::new();

        Self{ player, dealer, deck}
    }

    fn round(&mut self) {
        self.player.clear_hand();
        self.dealer.clear_hand();
        self.deck = Deck::new();

        self.player.player_money(0);
        let bet_amount: i32 = Player::bet(&self.player);

        for _ in 0..2 {
            self.deck.deal_card(&mut self.player.hand);
            self.deck.deal_card(&mut self.dealer.hand);
        }

        if self.player.has_bust() || self.dealer.has_bust() {
            return;
        }

        if self.player.has_blackjack() && self.dealer.has_blackjack() {
            println!("Both player and dealer have blackjack! It's a tie.");
            return;
        }

        if self.player.has_blackjack() {
            self.player.player_money(bet_amount);
            return;
        }

        if self.dealer.has_blackjack() {
            self.player.player_money(-bet_amount);
            return;
        }
        self.player.see_hand();

        loop {
            let mut choice: String = String::new();
            println!("Do you want to hit or stay? (h/s)");
            io::stdin().read_line(&mut choice).expect("Failed to read line");
            let choice= choice.trim().to_lowercase();

            match choice.as_str() {
                "h" | "hit" => {
                    self.deck.deal_card(&mut self.player.hand);
                    self.player.see_hand();
                    if self.player.has_bust() {
                        self.player.player_money(-bet_amount);
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
        while self.dealer.check_hand_value() < 17 {
            self.deck.deal_card(&mut self.dealer.hand);
            if self.dealer.has_bust(){
                self.player.player_money(bet_amount);
                return;
            }
        }
        let player_total = self.player.check_hand_value();
        let dealer_total = self.dealer.check_hand_value();
        
        self.dealer.see_hand();

        if dealer_total > player_total {
            println!("Dealer wins!, you lost ${}.", bet_amount);
            self.player.player_money(-bet_amount);
        } else if dealer_total < player_total {
            println!("Player wins!, you won ${}.", bet_amount * 2);
            self.player.player_money(bet_amount);
        } else {
            println!("It's a tie!");
        }

    }

    fn play_again() -> bool{
        loop{
        let mut play_again: String = String::new();
        println!("Do you want to play again? (y/n)");
        io::stdin().read_line(&mut play_again).expect("Failed to read line");
        let play_again = play_again.trim().to_lowercase();
        
        match play_again.as_str() {
            "y" | "yes" => {
                println!("Starting a new game...");
                return true;    
            },
            "n" | "no" => {
                println!("Thanks for playing!");
                return false;           
            },
            _ => {
                println!("Invalid choice. Please enter 'y' or 'n'.");            
            }
            }
        }
    }

    fn play_round(&mut self) {
        loop{
            self.round();
            if !Self::play_again() {
                break;
            }

        }
    }
}

fn main() {
    let mut game = Game::new();
    game.play_round();
}
