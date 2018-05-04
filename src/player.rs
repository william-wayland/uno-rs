use deck::*;
use card::*;

use std::fmt;
use util::*;

#[derive(Debug)]
pub struct Player {
    id: u8,
    name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(id: u8, name: String, deck: &mut Deck) -> Player {
        Player{id: id, name: name, hand: deck.new_hand()}
    }

    pub fn has_cards(&self) -> bool {
        match self.hand.len() {
            0 => true,
            _ => false,
        }
    }

    pub fn choose_card(&self) -> Option<usize> {
        loop {
            let input = read_line();
            if input == "s" {
                return None;
            } else {
                match input.parse::<usize>() {
                    Ok(input) => return Some(input),
                    Err(_e) => println!("That wasn't a number, was it?"),
                };
            }
        }           
    }

    pub fn peak_at_card(&self, index: usize) -> &Card {
        &self.hand[index]
    }

    pub fn take_card(&mut self, i: usize) -> Card {
        self.hand.remove(i)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn give_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn print_hand(&self) {
        println!();
        println!("{} your current hand is...", self.name);
        for (i, ref x) in self.hand.iter().enumerate() {
            println!("{}: {}", i, x);
        }
    }

    pub fn pick_colour() -> Colour {
        println!("You put down a Wild Card. You need to pick a colour.");
        println!("\t0. Red");
        println!("\t1. Green");
        println!("\t2. Blue");
        println!("\t3. Yellow");
        match read_line().parse().unwrap() {
            0 => Colour::Red,
            1 => Colour::Green,
            2 => Colour::Blue,
            3 => Colour::Yellow,
            _ => {
                //TODO Error handling
                Colour::Red
            }
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Player {}", self.id)?;
        for (i, card) in self.hand.iter().enumerate() {
            writeln!(f, "{}: {}", i, card)?;
        }
        writeln!(f)
    }
}