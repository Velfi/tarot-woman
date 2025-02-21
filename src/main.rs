use rand::prelude::*;
use std::io::Write;
use std::num::NonZeroU32;

mod cards;

fn main() {
    let mut rng = rand::rng();
    let mut deck = [cards::MAJOR_ARCANA, cards::MINOR_ARCANA].concat();
    deck.shuffle(&mut rng);

    let pam = image::load_from_memory(include_bytes!("../images/PamelaColmanSmith.jpg")).unwrap();
    let ascii_art = artem::convert(
        pam,
        &artem::config::ConfigBuilder::new()
            .target_size(NonZeroU32::new(38).unwrap())
            .build(),
    );
    let std_out = std::io::stdout();
    std_out.lock().write_all(ascii_art.as_bytes()).unwrap();

    let num_cards: usize;
    loop {
        println!("How many cards would you like to draw?");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        match user_input.trim().parse::<usize>() {
            Ok(num) => {
                num_cards = num;
                break;
            }
            Err(_) => {
                println!("Please enter a positive number no greater than 78");
                continue;
            }
        }
    }

    let cares: bool;
    loop {
        println!("Do you care which cards are inverted?");
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();

        cares = match user_input.trim() {
            "y" | "Y" | "yes" | "Yes" | "YES" => true,
            "n" | "N" | "no" | "No" | "NO" => false,
            _ => {
                println!("Please enter 'yes' or 'no'");
                continue;
            }
        };
        break;
    }

    let card_height = NonZeroU32::new(32).unwrap();
    deck.into_iter().take(num_cards).for_each(|card| {
        let inverted = rng.random::<bool>();
        println!("{}", card.wiki_image(card_height, inverted && cares));
        println!(
            "{}{}\n{}\n",
            card,
            if inverted && cares { " (inverted)" } else { "" },
            card.wiki_image_link()
        );
    });
}
