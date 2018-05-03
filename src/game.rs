use card::*;
use deck::*;
use player::*;

use std::fmt;
use rand::{Rng, thread_rng};
use util::*;

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

    pub fn check_winner(&self) -> Option<u8> {
        match self.players[self.turn as usize].has_cards() {
            true => Some(self.turn),
            false => None,
        }
    }

    pub fn game_loop(mut self) -> u8 {
        loop {
            // ======
            // The start of a new turn.
            // ======
            self.players[self.turn as usize].print_hand();
            println!("Pick a card, or enter \'s\' to skip turn");
            println!();
            println!("The top of the stack is a {}", self.stack[self.stack.len() - 1]);
            println!();

            //====
            // Checking user input
            //===
            loop {
                if let Some(index) = self.players[self.turn as usize].choose_card() {
                    if index == 0  { // TODO Replace with is_legal()
                        // Legal move
                        let card = self.players[self.turn as usize].take_card(index);
                        self.stack.push(card);

                        match self.check_winner() {
                            Some(_turn) => return self.turn,
                            None => { 
                                // ====
                                //  Turn Finished
                                // ====
                                break;
                            }
                        };
                        
                    } else {
                        // It was'nt a legal move
                        println!("That wasn't a legal move. Try again.");
                        continue;
                    }
                } 

                else {
                    //TODO loop this for multicard pickup (+2 and +4)
                    let card = self.deck.get_card();
                    println!("{} just picked up a {}", self.current_player_name(), card);
                    self.players[self.turn as usize].give_card(card);
                    
                    // After drawing new card from deck, it's the next persons turn.
                    break; 
                }
            }

            self.next_turn();
        }
    }

    pub fn current_player_name(&mut self) -> String {
        self.players[self.turn as usize].get_name()
    }

    pub fn current_player_id(&mut self) -> u8 {
        self.players[self.turn as usize].get_id()
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