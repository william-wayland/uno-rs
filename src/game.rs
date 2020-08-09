use card::*;
use deck::*;
use player::*;

use std::fmt;
use rand::{Rng, thread_rng};
use util::*;

#[allow(dead_code)] // for future feature where, as an option, a player can play after recieving a +n card.
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
        let number_of_players = Game::handle_number_of_players(8u8); // TODO: Config
        println!();
    
        let mut players: Vec<Player> = Vec::with_capacity(number_of_players as usize);
        for id in 0..number_of_players {
            println!("Player {} name?", id + 1u8); // People understand numbers starting from 1 better, so just... do that
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
        // First card on the deck (must have a colour)
        // ========
        let mut stack: Vec<Card> = Vec::with_capacity(106);
        loop {
            let card = deck.get_card().unwrap();
            
            match card.card_type {
                CardType::Wild | CardType::WildFour => {
                    stack.push(card);
                },
                _ => {
                    stack.push(card);
                    break;
                }
            };
        }

        // ====
        // Let's play
        // ====
        Game{players, stack, deck, turn, turn_direction, pickups: None}
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

    fn addon_pickup(&self, pickup_addition: usize) -> Option<usize> {
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
            CardType::DrawTwo => {}, 
            CardType::Wild | CardType::WildFour => card.colour = Player::pick_colour(),
            CardType::Number => {},  // Nothing. Added for completion
        };

        self.stack.push(card);
    }

    // If there are current pickups, handle that 
    // Else check if the player should skip turn, and handle it. 
    // Returns the turn direction -- TODO -- turn direction after pickups via game options
    fn handle_skip_move(&mut self) -> Option<Turn> {
        match self.pickups {
            Some(i) => {
                for _ in 0..i {
                    let card = self.pick_up_card();
                    println!("{} just picked up a {}", self.current_player_name(), card);
                    self.players[self.turn as usize].give_card(card);
                }

                self.pickups = None;

                return Some(Turn::New)   
            },
            None => {
                if self.can_current_player_move() {
                    println!("You don't need to skip! You must play a card.");
                    return None
                } else {
                    let card = self.pick_up_card();
                    println!("{} just picked up a {}", self.current_player_name(), card);
                    self.players[self.turn as usize].give_card(card);
                    return Some(Turn::New)
                }              
            }
        }
    }

    fn handle_number_of_players(max: u8) -> u8 {
        loop {
            let input = read_line();
            let input: u8 = match input.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("That doesn't appear to be a number. Try again.");
                    continue;
                }
            };

            if input > max {
                println!("The maximum number of players is set to {}. Try again.", max);
                continue;
            } else if input < 2 {
                println!("The minimum number of players is set to 2. Try again.");
                continue;
            } else {
                return input
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
                None => println!("Pick a card, or enter \'s\' to skip turn and pick up a card."),
            };
            println!();
            println!("The top of the stack is a {}", self.top_card());
            println!();

            match self.handle_turn() {
                Some(winner) => return winner,
                None => ()
            };

            self.next_turn();
        }
    }

    pub fn handle_turn(&mut self) -> Option<String> {
        loop { 
            if let Some(index) = self.players[self.turn as usize].choose_card() {
                let (is_legal, pickup_addition) = Game::is_legal_move(
                    self.top_card(), 
                    self.current_player().peak_at_card(index), 
                    &self.pickups);
                
                if is_legal { 
                    self.pickups = self.addon_pickup(pickup_addition);
                    self.handle_legal_move(index);

                     match self.check_winner() {
                        Some(_) => return Some(self.current_player().get_name()),
                        None => return None, // Next turn
                    };
                } 
                else {
                    println!("That wasn't a legal move. Try again.");
                }
            } 
            else {
                match self.handle_skip_move() {
                    Some(turn) => {
                        match turn {
                            Turn::Again => continue,
                            Turn::New => return None
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
        } 
    }

    // Returns (if_legal, +pickup, )
    pub fn is_legal_move(stack: &Card, player: &Card, pickup: &Option<usize>) -> (bool, usize){
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
                if stack.colour == player.colour || stack.card_type == CardType::DrawTwo {  
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
        self.current_player().get_name()
    }

    // Deck::get_card should only give None when it has run out of card to give
    // Thus, we need to reshuffle the stack(-the top card) into the deck.
    // Then we can take the card from the newly reshuffled deck.
    pub fn pick_up_card(&mut self) -> Card {
        match self.deck.get_card() {
            Some(card) => card,
            None => {
                let top = self.stack.pop().unwrap();
                self.deck.reshuffle_deck(&mut self.stack);
                self.stack.push(top);
                self.deck.get_card().unwrap()
            }
        }
    }

    pub fn can_current_player_move(&mut self) -> bool {
        for i in 0..self.current_player().number_of_cards() {
            let (legal, _) = Game::is_legal_move(self.top_card(), self.current_player().peak_at_card(i), &None);
            if legal {
                return true;
            }
        }
        return false;
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.turn as usize]
    }
    

    pub fn top_card(&self) -> &Card {
        &self.stack[self.stack.len() - 1]
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        //for player in self.players.iter() {
        //    write!(f, "{}\n", player)?;
        //}

        // Now, it should always have something on the stack.
        // writeln!(f, "Top of stack: {}", self.stack[self.stack.len() - 1])?;
        writeln!(f, "Stack: {:?}", self.stack)?;
        writeln!(f, "Turn direction: {}", self.turn_direction)?;
        writeln!(f, "Turn: {}", self.turn)
    }
}