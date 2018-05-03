extern crate rand;
use rand::{Rng};

use card::*;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(108);

        Deck::generate_cards(&mut cards);

        rand::thread_rng().shuffle(&mut cards);

        Deck{cards}
    }

    pub fn generate_cards(cards: &mut Vec<Card>) {
        for i in 0..=9 {
            cards.push(Card::new(Colour::Red, Some(i), CardType::Number));
            cards.push(Card::new(Colour::Green, Some(i), CardType::Number));
            cards.push(Card::new(Colour::Blue, Some(i), CardType::Number));
            cards.push(Card::new(Colour::Yellow, Some(i), CardType::Number));
        }
        for i in 1..=9 {
            cards.push(Card::new(Colour::Red, Some(i), CardType::Number));
            cards.push(Card::new(Colour::Green, Some(i), CardType::Number));
            cards.push(Card::new(Colour::Blue, Some(i), CardType::Number));
            cards.push(Card::new(Colour::Yellow, Some(i), CardType::Number));
        }
        for _ in 0..2 {
            cards.push(Card::new(Colour::Red, None, CardType::Reverse));
            cards.push(Card::new(Colour::Green, None, CardType::Reverse));
            cards.push(Card::new(Colour::Blue, None, CardType::Reverse));
            cards.push(Card::new(Colour::Yellow, None, CardType::Reverse));

            cards.push(Card::new(Colour::Red, None, CardType::Skip));
            cards.push(Card::new(Colour::Green, None, CardType::Skip));
            cards.push(Card::new(Colour::Blue, None, CardType::Skip));
            cards.push(Card::new(Colour::Yellow, None, CardType::Skip));

            cards.push(Card::new(Colour::Red, None, CardType::DrawTwo));
            cards.push(Card::new(Colour::Green, None, CardType::DrawTwo));
            cards.push(Card::new(Colour::Blue, None, CardType::DrawTwo));
            cards.push(Card::new(Colour::Yellow, None, CardType::DrawTwo));
        }
        for _ in 0..3 {
            cards.push(Card::new(Colour::Black, None, CardType::Wild));
            cards.push(Card::new(Colour::Black, None, CardType::WildFour));
        }
    }

    pub fn new_hand(&mut self) -> Vec<Card> {
        let mut hand: Vec<Card> = Vec::with_capacity(7);
        self.fill_with_cards(&mut hand, 7);
        hand
    }

    pub fn get_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn fill_with_cards(&mut self, input: &mut Vec<Card>, number_of_cards: u8) {
        for _ in 0..number_of_cards {
            input.push(self.cards.pop().unwrap());
        }
    }
}