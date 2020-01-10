use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CardType {
    Number, DrawTwo, Skip, Reverse, Wild, WildFour
}

#[derive(Debug, PartialEq)]
pub enum Colour {
    Red, Green, Blue, Yellow, Black
}

#[derive(Debug)]
pub struct Card {
    pub colour: Colour,
    pub number: Option<u8>,
    pub card_type: CardType,
}

impl Card {
    pub fn new(colour: Colour, number: Option<u8>, card_type: CardType) -> Card {
        Card{colour, number, card_type}
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self.colour {
            Colour::Black => write!(f, "{:?} Card", self.card_type),
            _ => {
                match self.card_type {
                    CardType::WildFour => write!(f, "{:?} Wild +4", self.colour),
                    CardType::Wild => write!(f, "{:?} Wild", self.colour),
                    CardType::Reverse => write!(f, "{:?} Reverse", self.colour),
                    CardType::Skip => write!(f, "{:?} Skip", self.colour),
                    CardType::DrawTwo => write!(f, "{:?} Draw Two", self.colour),
                    CardType::Number => {
                        if let Some(number) = self.number { 
                            write!(f, "{:?} {:?}", self.colour, number)?;
                        }
                        write!(f, "")
                    },
                }
            }
        }     
    }
}
