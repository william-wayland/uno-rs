/**
    @Auther         William Wayland
    @Auther Contact william.wayland@griffithuni.edu.au
**/

use std::fmt;

extern crate rand;
use rand::Rng;

#[derive(Debug)]
enum Colour {
    Red, Green, Blue, Yellow
}

#[derive(Debug)]
enum CardType {
    Number, DrawTwo, Skip, Reverse, Wild, WildFour
}

#[derive(Debug)]
struct Card {
    colour: Option<Colour>,
    number: Option<u8>,
    card_type: CardType,
}

impl Card {
    fn new(colour: Option<Colour>, number: Option<u8>, card_type: CardType) -> Card {
        Card{colour, number, card_type}
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\tType: {:?}, Number: {:?}, Colour: {:?}", self.card_type, self.number, self.colour)
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::with_capacity(108);

        for i in 0..=9 {
            cards.push(Card::new(Some(Colour::Red), Some(i), CardType::Number));
            cards.push(Card::new(Some(Colour::Green), Some(i), CardType::Number));
            cards.push(Card::new(Some(Colour::Blue), Some(i), CardType::Number));
            cards.push(Card::new(Some(Colour::Yellow), Some(i), CardType::Number));
        }
        for i in 1..=9 {
            cards.push(Card::new(Some(Colour::Red), Some(i), CardType::Number));
            cards.push(Card::new(Some(Colour::Green), Some(i), CardType::Number));
            cards.push(Card::new(Some(Colour::Blue), Some(i), CardType::Number));
            cards.push(Card::new(Some(Colour::Yellow), Some(i), CardType::Number));
        }
        for _ in 0..2 {
            cards.push(Card::new(Some(Colour::Red), None, CardType::Reverse));
            cards.push(Card::new(Some(Colour::Green), None, CardType::Reverse));
            cards.push(Card::new(Some(Colour::Blue), None, CardType::Reverse));
            cards.push(Card::new(Some(Colour::Yellow), None, CardType::Reverse));

            cards.push(Card::new(Some(Colour::Red), None, CardType::Skip));
            cards.push(Card::new(Some(Colour::Green), None, CardType::Skip));
            cards.push(Card::new(Some(Colour::Blue), None, CardType::Skip));
            cards.push(Card::new(Some(Colour::Yellow), None, CardType::Skip));

            cards.push(Card::new(Some(Colour::Red), None, CardType::DrawTwo));
            cards.push(Card::new(Some(Colour::Green), None, CardType::DrawTwo));
            cards.push(Card::new(Some(Colour::Blue), None, CardType::DrawTwo));
            cards.push(Card::new(Some(Colour::Yellow), None, CardType::DrawTwo));
        }
        for _ in 0..3 {
            cards.push(Card::new(None, None, CardType::Wild));
            cards.push(Card::new(None, None, CardType::WildFour));
        }

        rand::thread_rng().shuffle(&mut cards);

        Deck{cards}
    }

    fn new_hand(&mut self) -> Vec<Card> {
        let mut hand: Vec<Card> = Vec::with_capacity(7);
        for _ in 0..=6 {
            hand.push(self.cards.pop().unwrap());
        }
        hand
    }
}

#[derive(Debug)]
struct Player {
    id: u8,
    hand: Vec<Card>,
}

impl Player {
    fn new(id: u8, deck: &mut Deck) -> Player {
        Player{id: id, hand: deck.new_hand()}
    }

    fn draw(&mut self, deck: &mut Deck, number_to_draw: u8) {
        for _ in 0..number_to_draw {
            self.hand.push(deck.cards.pop().unwrap());
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player {}\n", self.id)?;
        for card in self.hand.iter() {
            write!(f, "{}\n", card)?;
        }
        write!(f, "\n")
    }
}

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
}


fn main() {
    let mut deck = Deck::new();
    let mut player1 = Player::new(1, &mut deck);
    println!("{}", player1);
    player1.draw(&mut deck, 3);
    println!("{}", player1);
}
