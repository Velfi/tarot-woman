use rand::prelude::*;

mod cards;

fn main() {
    let mut rng = rand::rng();
    let mut deck = [cards::MAJOR_ARCANA, cards::MINOR_ARCANA].concat();
    deck.shuffle(&mut rng);

    let num_cards: usize;
    loop {
        println!("How many cards would you like to draw?");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        match user_input.trim().parse::<usize>() {
            Ok(num) => {
                num_cards = num;
                break;
            },
            Err(_) => {
                println!("Please enter a positive number no greater than 78");
                continue;
            },
        }
    }

    let upside_down: bool;
    loop {
        println!("Do you care which cards are inverted?");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        upside_down = match user_input.trim() {
            "y" | "Y" | "yes" | "Yes" | "YES" => true,
            "n" | "N" | "no" | "No" | "NO" => false,
            _ => {
                println!("Please enter 'yes' or 'no'");
                continue
            },
        };
        break;
    }

    deck.into_iter().take(num_cards).for_each(|card| {
        if upside_down {
            let orientation = if rng.random::<bool>() {
                " (inverted) "
            } else { " " };
            println!("{}{}{}", card, orientation, card.wiki_image_link() );
        } else {
            println!("{} {}", card, card.wiki_image_link());
        }
    });
}
