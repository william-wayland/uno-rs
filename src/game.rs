use card::*;
use deck::*;
use player::*;

use std::fmt;
use rand::{Rng, thread_rng};
use util::*;

#[derive(Debug, PartialEq)]
enum Turn {
    New, Again
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    deck: Deck,
    stack: Vec<Card>,
    turn: u8,
    turn_direction: bool,
    pickups: Option<usize>
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
        let pickups = None;

        // ====
        // Let's play
        // ====
        Game{players, stack, deck, turn, turn_direction, pickups}
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


    fn calculate_pickup(&self, pickup_addition: usize) -> Option<usize> {
        match self.pickups {
            Some(pickups) => {
                Some(pickups + pickup_addition)
            },
            None => {
                if pickup_addition > 0 {
                    Some(pickup_addition)
                } else {
                    None
                }
            },
        }
    }

    fn handle_legal_move(&mut self, index: usize) {
        let mut card = self.players[self.turn as usize].take_card(index);

        match card.card_type {
            CardType::Reverse => self.turn_direction = !self.turn_direction,
            CardType::Skip => self.next_turn(), // other next turn after break at the break inner loop.
            CardType::DrawTwo => {}, // TODO implement extra pickup
            CardType::Wild | CardType::WildFour => {
                card.colour = Player::pick_colour();
            }, 
            CardType::Number => {},  // Nothing. Added for completion
        };

        self.stack.push(card);
    }

    fn handle_skip_move(&mut self) -> Turn {
        match self.pickups {
            Some(i) => {
                for _ in 0..i {
                    let card = self.deck.get_card();
                    println!("{} just picked up a {}", self.current_player_name(), card);
                    self.players[self.turn as usize].give_card(card);
                }

                self.pickups = None;

                let option_multipickup_victom = true;
                if option_multipickup_victom {
                    println!("Your turn again, {}.", self.players[self.turn as usize].get_name());
                    self.players[self.turn as usize].print_hand();

                    println!();
                    println!("The top of the stack is a {}", self.stack[self.stack.len() - 1]);
                    println!();
                    Turn::Again
                } else {
                    Turn::New
                }
            },
            None => {
                let card = self.deck.get_card();
                println!("{} just picked up a {}", self.current_player_name(), card);
                self.players[self.turn as usize].give_card(card);
                Turn::New
            }
        }
    }

    pub fn game_loop(mut self) -> String {
        loop {
            // ======
            // The start of a new turn.
            // ======
            self.players[self.turn as usize].print_hand();
            match self.pickups {
                Some(i)  => println!("\nWARNING: You're at risk of picking up {} cards.\nEnter a +pickup card to pass it or \'s\' to accept it.", i),
                None => println!("Pick a card, or enter \'s\' to skip turn"),
            };
            println!();
            println!("The top of the stack is a {}", self.stack[self.stack.len() - 1]);
            println!();

            loop { // Breaking out of this loop means a new turn.
                if let Some(index) = self.players[self.turn as usize].choose_card() {
                    let (is_legal, pickup_addition) = Game::is_legal_move(&self.stack[self.stack.len() - 1], self.players[self.turn as usize].peak_at_card(index), &self.pickups);
                    self.pickups = self.calculate_pickup(pickup_addition);

                    if is_legal { 
                        self.handle_legal_move(index);

                         match self.check_winner() {
                            Some(_) => return self.players[self.turn as usize].get_name(),
                            None => break, // Next turn
                        };
                    } 
                    else {
                        println!("That wasn't a legal move. Try again.");
                    }
                } 
                else {
                    match self.handle_skip_move() {
                        Turn::Again => continue,
                        Turn::New => break,
                    }
                }
            } // End of inner loop

            self.next_turn();
        }
    }

    // Returns (if_legal, +pickup, )
    pub fn is_legal_move(stack: &Card, player: &Card, pickup: &Option<usize>) -> (bool, usize){
        // TODO Should the move be allowed to happen?
        // TODO chnage the colour of the card if it's black

        match player.card_type {
            CardType::WildFour => {
                (true, 4)
            },
            CardType::Wild => {
                if let Some(_) = &pickup {
                    return (false, 0)
                }
                (true, 0)
            },
            CardType::DrawTwo => {
                // Assuming right colour, can be placed on anything.
                if stack.colour == player.colour {  
                    (true, 2)
                } else {
                    (false, 0)
                }
            },
            CardType::Reverse => {
                 // Assuming right colour, can be placed on anything.
                if stack.colour == player.colour || stack.card_type == CardType::Reverse {  
                    (true, 0)
                } else {
                    (false, 0)
                }
            },
            CardType::Skip => {
                 // Assuming right colour, can be placed on anything.
                if stack.colour == player.colour || stack.card_type == CardType::Skip {  
                    (true, 0)
                } else {
                    (false, 0)
                }
            },
            CardType::Number => {
                if let Some(_) = &pickup {
                    (false, 0)
                } else {
                    if stack.colour == player.colour || stack.number == player.number {  
                        (true, 0)
                    } else {
                        (false, 0)
                    }
                }
            },
        }
    }

    pub fn current_player_name(&mut self) -> String {
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