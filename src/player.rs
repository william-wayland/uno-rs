use deck::*;
use card::*;

use std::fmt;

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

    #[allow(dead_code)]
    pub fn draw(&mut self, deck: &mut Deck, number_to_draw: u8) {
        deck.fill_with_cards(&mut self.hand, number_to_draw);
    }

    pub fn has_cards(&mut self) -> bool{
        match self.hand.len() {
            0 => true,
            _ => false,
        }
    }

    pub fn get_name(&mut self) -> String {
        self.name.clone()
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