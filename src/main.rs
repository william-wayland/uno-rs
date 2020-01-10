/**
    @Auther         William Wayland
    @Auther Contact w.g.wayland@gmail.com
**/

extern crate rand;
extern crate colored;

mod card;
mod deck;
mod player;
mod game;
mod util;

use game::*;

fn main() {

    let game = Game::new();
    let winner = game.game_loop();

    println!("Congratulations {}!", winner);

}
