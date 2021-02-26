use std::io;
use rand::Rng;

enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
    Ace,
}

fn bet_once(balance: i32) -> i32 {
    // Contains logic for a bet.  Returns the amount that the player won or lost.
    let mut card1: i32 = 0;
    let mut card2: i32 = 0;

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

    let mut input = String::new();
    let mut bet: i32 = 0;

    loop {
        println!("Your cards are: {} and {}", card1, card2);

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
            break;
        }
    }

    // XXX not quite the right distribution
    let mut card3: i32 = 0;
    loop {
        card3 = rand::thread_rng().gen_range(0, 12);
        if card1 == card3 || card2 == card3 {
            continue;
        } else {
            break;
        }
    }

    println!("The card that was drawn is {}", card3);
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
    // put explanation here

    while balance > 0 {
        let change = bet_once(balance);
        if change > 0 {
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
