use std::fmt;

#[derive(Debug)]
pub enum CardType {
    Number, DrawTwo, Skip, Reverse, Wild, WildFour
}

#[derive(Debug)]
pub enum Colour {
    Red, Green, Blue, Yellow
}

#[derive(Debug)]
pub struct Card {
    colour: Option<Colour>,
    number: Option<u8>,
    card_type: CardType,
}

impl Card {
    pub fn new(colour: Option<Colour>, number: Option<u8>, card_type: CardType) -> Card {
        Card{colour, number, card_type}
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {


        if let Some(ref number) = self.number  {
            if let Some(ref colour) = self.colour {
                write!(f, "{:?} {:?}", colour, number)
            } else {
                write!(f, "")
            }
        } 
        else if let Some(ref colour) = self.colour {
            write!(f, "{:?} {:?}", colour, self.card_type)
        } 
        else {
            write!(f, "A {:?}", self.card_type)
        }

    }
}
