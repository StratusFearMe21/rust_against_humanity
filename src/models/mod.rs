use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Cards {
    pub name: String,
    pub white: Vec<WhiteCard>,
    pub black: Vec<BlackCard>,
    pub official: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WhiteCard {
    pub text: String,
    pub pack: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlackCard {
    pub text: String,
    pub pick: u8,
    pub pack: u8,
}
