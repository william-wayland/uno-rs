/**
    @Auther         William Wayland
    @Auther Contact william.wayland@griffithuni.edu.au
**/

use std::fmt;

extern crate rand;
use rand::{Rng, thread_rng};

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

        Deck::generate_cards(&mut cards);

        rand::thread_rng().shuffle(&mut cards);

        Deck{cards}
    }

    fn generate_cards(cards: &mut Vec<Card>) {
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
    }

    fn new_hand(&mut self) -> Vec<Card> {
        let mut hand: Vec<Card> = Vec::with_capacity(7);
        self.fill_with_cards(&mut hand, 7);
        hand
    }

    fn get_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    fn fill_with_cards(&mut self, input: &mut Vec<Card>, number_of_cards: u8) {
        for _ in 0..number_of_cards {
            input.push(self.cards.pop().unwrap());
        }
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

    #[allow(dead_code)]
    fn draw(&mut self, deck: &mut Deck, number_to_draw: u8) {
        deck.fill_with_cards(&mut self.hand, number_to_draw);
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

#[derive(Debug)]
struct Game {
    players: Vec<Player>,
    deck: Deck,
    stack: Vec<Card>,
    turn: u8,
    turn_direction: bool,
}

impl Game {
    fn new(number_of_players: u8) -> Game {
        //make Deck

        let mut deck = Deck::new();

        // make players
        let mut players: Vec<Player> = Vec::with_capacity(number_of_players as usize);
        for id in 0..number_of_players {
            players.push(Player::new(id, &mut deck));
        }

        // set turn to something random
        let turn: u8 = thread_rng().gen::<u8>() % number_of_players;

        let mut stack: Vec<Card> = Vec::with_capacity(30);
        stack.push(deck.get_card());

        let turn_direction = false;

        Game{players, stack, deck, turn, turn_direction}
    }


    fn next_turn(&mut self) {
        if self.turn_direction {
            self.turn = (self.turn + 1) % self.players.len() as u8;
        }
        else {
            // Underflow protection.
           if self.turn == 0 {
               self.turn = (self.players.len() - 1 )as u8;
           }
           else {
               self.turn -= 1;
           }
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        //for player in self.players.iter() {
        //    write!(f, "{}\n", player)?;
        //}

        // Now, it should always have something on the stack.
        writeln!(f, "Top of stack: {}", self.stack[self.stack.len() - 1])?;
        writeln!(f, "Turn direction: {}", self.turn_direction)?;
        writeln!(f, "Turn: {}", self.turn)
    }
}


fn main() {

    let mut game = Game::new(4);
    println!("{}", game);
    game.next_turn();
    println!("{}", game);
    game.next_turn();
    println!("{}", game);
    game.next_turn();
    println!("{}", game);

}
