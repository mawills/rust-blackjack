mod card;

use card::{ Card, Suit };
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    // Initialize deck
    let mut deck = create_deck();
    deck.shuffle(&mut thread_rng());

    // Initialize hands
    let mut dealer_hand = Vec::new();
    let mut player_hand = Vec::new();

    // Deal out starting hands
    draw_cards(1, &mut dealer_hand, &mut deck);
    draw_cards(1, &mut player_hand, &mut deck);
    draw_cards(1, &mut dealer_hand, &mut deck);
    draw_cards(1, &mut player_hand, &mut deck);

    println!("~~~Game Start~~~");

    println!("Dealer's hand:");
    print_hand(&dealer_hand);

    println!("Player's hand:");
    print_hand(&player_hand);

    // Sum the total points of the starting hands
    let mut dealer_score = sum_points(&dealer_hand);
    let mut player_score = sum_points(&player_hand);

    // Game loop for the player's turn
    loop {
        println!("~~~Player's Turn~~~");
        println!("Would you like to Hit (h) or Stand (s)?: ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error: unable to read user input");

        println!("");

        match input.trim() {
            "h" => {
                draw_cards(1, &mut player_hand, &mut deck);
    
                player_score = sum_points(&player_hand);

                if player_score > 21 {
                    break
                }
    
                println!("Dealer's hand:");
                print_hand(&dealer_hand);
    
                println!("Player's hand:");
                print_hand(&player_hand);
            },
            "s" => {
                println!("Player stands with a score of {}", sum_points(&player_hand));
                break
            }
            _ => {
                println!("Sorry, I don't understand that input. Try again.");
            }
        }
    }

    // Game loop for the dealer's turn
    while dealer_score <= 21 {
        println!("~~~Dealer's Turn~~~");

        // Dealer is greedy and will take any chance to beat or tie the player if it's losing
        if dealer_score < player_score && player_score <= 21 && dealer_score != 21 {
            println!("Dealer Hits with a current score of {}", sum_points(&dealer_hand));
            draw_cards(1, &mut dealer_hand, &mut deck);
            dealer_score = sum_points(&dealer_hand);
        } else {
            println!("Dealer Stands with a score of {}", sum_points(&dealer_hand));
            break
        }

        println!("Dealer's hand:");
        print_hand(&dealer_hand);

        println!("Player's hand:");
        print_hand(&player_hand);
    }

    print_result(player_score, dealer_score)
}

/// Returns a standard un-shuffled 52-card playing card deck
fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    for i in 1..13 {
        deck.push(Card { suit: Suit::Spades, value: i });
        deck.push(Card { suit: Suit::Hearts, value: i });
        deck.push(Card { suit: Suit::Diamonds, value: i });
        deck.push(Card { suit: Suit::Clubs, value: i });
    }

    deck
}

/// Draws n cards into the provided hand from the provided deck, removing them from the deck
fn draw_cards(n: usize, hand: &mut Vec<Card>, deck: &mut Vec<Card>){
    for _ in (deck.len() - n) .. deck.len() {
        hand.push(deck.pop().unwrap());
    }
}

/// Prints the contents of a provided hand to the console
fn print_hand(hand: &Vec<Card>) {
    for card in hand {
        match card.value {
            1 => println!("{} of {:?}", "Ace", card.suit),
            11 => println!("{} of {:?}", "Jack", card.suit),
            12 => println!("{} of {:?}", "Queen", card.suit),
            13 => println!("{} of {:?}", "King", card.suit),
            _ => println!("{} of {:?}", card.value, card.suit)
        }
    }
    println!("\n");
}

/// Returns the maximum point value of a hand without exceeding 21 points
fn sum_points(hand: &Vec<Card>) -> u8 {
    let mut num_aces = 0;
    for card in hand {
        if card.value == 1 {
            num_aces += 1;
        }
    }

    let mut sum = 0;
    for card in hand {
        // Treat Aces as 11 points by default
        if card.value == 1 {
            sum += 11;
        }

        // All face cards are worth 10 points
        else if card.value > 10 {
            sum += 10;
        }

        else {
            sum += card.value;
        }
    }

    // Treat Aces as 1 instead of 11 if it would put the hand over 21 points
    while sum > 21 && num_aces > 0 {
        sum -= 10;
        num_aces -= 1;
    }

    sum
}

/// Compares the final scores and prints the result of the game to the console
fn print_result(player_score: u8, dealer_score: u8) {
    println!("~~~GAME OVER~~~");

    if player_score == dealer_score {
        println!("Tie with a score of {} to {}!", player_score, dealer_score);
    }

    else if player_score > 21 && dealer_score <= 21 {
        println!("Player busts! Dealer wins with a score of {} to {}!", dealer_score, player_score);
    }

    else if dealer_score > 21 && player_score <= 21 {
        println!("Dealer busts! Player wins with a score of {} to {}!", player_score, dealer_score);
    }

    else if player_score > 21 && dealer_score > 21 {
        println!("Both players bust! Tie with a score of {} to {}!", player_score, dealer_score);
    }

    else {
        let high_score = std::cmp::max(player_score, dealer_score);
        if player_score == high_score {
            println!("Player wins with score of {} to {}!", player_score, dealer_score);
        }

        if dealer_score == high_score {
            println!("Dealer wins with score of {} to {}!", dealer_score, player_score);
        }
    }
}
