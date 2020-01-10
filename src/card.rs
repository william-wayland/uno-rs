use std::fmt;
use colored::*;

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
            Colour::Black => {
                match self.card_type {
                    CardType::Wild => write!(f, "Wild"),
                    CardType::WildFour => write!(f, "Wild +4"),
                    _ => write!(f, "Something has gone terribly wrong."),
                }
            }
            _ => {
                let mut o = String::new();

                match self.colour {
                    Colour::Red => o = format!("{}{:?}", o, Colour::Red),
                    Colour::Green => o = format!("{}{:?}", o, Colour::Green),
                    Colour::Blue => o = format!("{}{:?}", o, Colour::Blue),
                    Colour::Yellow => o = format!("{}{:?}", o, Colour::Yellow),
                    _ => ()
                };

                match self.card_type {
                    CardType::WildFour => o = format!("{} Wild +4", o),
                    CardType::Wild => o = format!("{} Wild", o),
                    CardType::Reverse => o = format!("{} Reverse", o),
                    CardType::Skip => o = format!("{} Skip", o),
                    CardType::DrawTwo => o = format!("{} Draw Two", o),
                    CardType::Number => {
                        if let Some(number) = self.number { 
                           o = format!("{} {:?}", o, number);
                        }
                    },
                };

                match self.colour {
                    Colour::Red => write!(f, "{}", o.red())?,
                    Colour::Green => write!(f, "{}", o.green())?,
                    Colour::Blue => write!(f, "{}", o.blue())?,
                    Colour::Yellow => write!(f, "{}", o.yellow())?,
                    _ => (),
                };
                
                write!(f, "")
            }
        }     
    }
}
