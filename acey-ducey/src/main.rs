// Implements Acey-Ducey, from Basic Computer Games (1978)

use std::io;
use rand::Rng;

fn integer_to_card(value: i32) -> &'static str {
    match value {
        0 => "a Two",
        1 => "a Three",
        2 => "a Four",
        3 => "a Five",
        4 => "a Six",
        5 => "a Seven",
        6 => "an Eight",
        7 => "a Nine",
        8 => "a Jack",
        9 => "a Queen",
        10 => "a King",
        11 => "an Ace",
        _ => "Unknown",
    }
}

fn choose_random_cards() -> (i32, i32) {
    // Choose two random cards, ensuring that they're different and that card1 is
    // lower-ranked than card2.
    let mut card1: i32;
    let mut card2: i32;

    loop {
        card1 = rand::thread_rng().gen_range(0, 12);
        card2 = rand::thread_rng().gen_range(0, 12);
        if card1 != card2 {
            break;
        }
    }

    // Ensure card1 is lower
    if card1 > card2 {
        let swap = card1;
        card1 = card2;
        card2 = swap;
    }

    return (card1, card2);
}

fn request_bet(balance: i32, card_msg: String) -> i32 {
    // Prompts the user for a bet amount.  Negative amounts aren't allowed, but zero is,
    // and they can't bet more than their balance.
    let mut input = String::new();
    let mut bet: i32;

    loop {
        println!("\nYour balance is: ${}", balance);
        println!("{}", card_msg);

        println!("How much would you like to bet?");
        io::stdin().read_line(&mut input).expect(
            "Failed to read line",
        );

        bet = input.trim().parse().expect("Please type a number!");

        if bet < 0 {
            println!("You can't bet a negative amount!");
            continue;
        } else if bet == 0 {
            println!("Chicken!");
            break;
        } else if bet > balance {
            println!("You can't bet more than your balance of ${}", balance);
            continue;
        } else {
            // Bet amount passed our tests
            break;
        }
    }

    // Return the bet value
    bet
}


fn bet_once(balance: i32) -> i32 {
    // Contains logic for a bet.  Returns the amount that the player won or lost.
    let (card1, card2) = choose_random_cards();

    let bet = request_bet(
        balance,
        format!(
            "Your cards are: {} and {}",
            integer_to_card(card1),
            integer_to_card(card2)
        ),
    );

    // XXX not quite the right distribution here -- oh well!
    let mut card3: i32;
    loop {
        card3 = rand::thread_rng().gen_range(0, 12);
        if card1 == card3 || card2 == card3 {
            continue;
        } else {
            break;
        }
    }

    println!("The card that was drawn is {}", integer_to_card(card3));
    if card1 < card3 && card3 < card2 {
        println!("You win!");
        return bet;
    } else {
        println!("You lose!");
        return 0 - bet;
    }
}


fn main() {
    let mut balance = 100;
    let mut bets_made = 0;

    println!("Let's play Acey-Ducey!");
    // put explanation of the game here

    while balance > 0 {
        let change = bet_once(balance);
        if change >= 0 {
            println!("\nYou won ${}", change);
        } else {
            println!("\nYou lost ${}", 0 - change);
        }
        balance = balance + change;
        println!("\nYour balance is now ${}", balance);
        bets_made = bets_made + 1;
    }

    println!(
        "You are now penniless after {} bets. Better luck next time!",
        bets_made
    );
}
