use card::*;
use deck::*;
use player::*;

use std;
use std::fmt;
use std::io::Write;
use rand::{Rng, thread_rng};


fn read_line() -> String {
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    deck: Deck,
    stack: Vec<Card>,
    turn: u8,
    turn_direction: bool,
}

impl Game {
    pub fn new() -> Game {

        let mut deck = Deck::new();

        // =============
        // Introduce Players
        // =============
        println!("How many players?");
        let input = read_line();
        let number_of_players: u8 = match input.parse() {
            Ok(n) => n,
            Err(_) => {
                2 // TODO: handle None case.
            }
        };
        println!();

        let mut players: Vec<Player> = Vec::with_capacity(number_of_players as usize);
        for id in 0..number_of_players {
            println!("Player {} name?", id + 1u8);
            let input = read_line();
            println!("Welcome {}.\n", input);
            players.push(Player::new(id, input, &mut deck));
        }

        // ==============
        // set up the turns
        // ==============
        let turn: u8 = thread_rng().gen::<u8>() % number_of_players;
        let turn_direction = false;

        // ========
        // First card on the deck
        // ========
        let mut stack: Vec<Card> = Vec::with_capacity(30);
        stack.push(deck.get_card());

        // ====
        // Let's play
        // ====
        Game{players, stack, deck, turn, turn_direction}
    }

    pub fn next_turn(&mut self) {
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

    pub fn check_winner(&mut self) -> Option<u8> {
        match self.players[self.turn as usize].has_cards() {
            true => Some(self.turn),
            false => None,
        }
    }

    pub fn game_loop(&mut self) -> u8 {
        //

        loop {

            // TODO: User input

            // TODO: Implement user input.
            
            // After each turn, check for a winner.
            match self.check_winner() {
                Some(turn) => return self.turn,
                None => self.next_turn(),
            };
        }
    }

    pub fn current_player(&mut self) -> String {
        self.players[self.turn as usize].get_name()
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