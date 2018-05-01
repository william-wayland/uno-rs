/**
    @Auther         William Wayland
    @Auther Contact william.wayland@griffithuni.edu.au
**/

extern crate rand;

use std::fmt;
use std::io::Write;

mod card;
mod deck;
mod player;
mod game;

use game::*;

fn main() {

    let mut game = Game::new();
    println!("{}", game.current_player());
    game.next_turn();
    println!("{}", game.current_player());
    game.next_turn();
    println!("{}", game.current_player());
    game.next_turn();
    println!("{}", game.current_player());



}
