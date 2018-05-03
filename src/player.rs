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

    pub fn draw(&mut self, deck: &mut Deck, number_to_draw: u8) {
        deck.fill_with_cards(&mut self.hand, number_to_draw);
    }

    pub fn has_cards(&self) -> bool {
        match self.hand.len() {
            0 => true,
            _ => false,
        }
    }

    pub fn choose_card(&self) -> Option<(&[Card], u8)> {
        // TODO: Handle error
        
        let input = read_line();
        if input == "s" {
            return None;
        }
        let input = input.parse().unwrap();
        Some((&self.hand, input))
    }

    pub fn take_card(&mut self, i: usize) -> Card {
        self.hand.remove(i)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn print_hand(&self) {
        println!();
        println!("{} your current hand is...", self.name);
        for (i, ref x) in self.hand.iter().enumerate() {
            println!("{}: {}", i, x);
        }
    }

    pub fn test_cheat(&mut self) {
        self.hand.clear();
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