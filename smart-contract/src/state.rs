// this file holds all the structs or states 
use anchor_lang::prelude::*;

#[account]
pub struct Master{
    pub last_bet_id:u64,
}

#[account]
pub struct Bet{
    //unique identifier per user
    pub id: u64,
    //how much it costs to make the bet in lamports
    pub amount: u64,
    //maker's prediction
    pub prediction_a: BetPrediction,
    //taker's prediction (NONE at bet creation)
    pub prediction_b: Option<BetPrediction>,
    //current state of the bet
    pub state: BetState,
    //pyth price oracle account
    pub pyth_price_key : Pubkey,
    //bet becomes invalid after theis UNIX timestamp
    pub expiry_ts: i64,
}

#[derive(AnchorSerialize , AnchorDeserialize , Clone)]
pub struct BetPrediction{
    //the address that bets
    pub player: Pubkey,
    //price prediction in USD
    pub price : f64,    
}

#[derive(AnchorSerialize , AnchorDeserialize , Clone , PartialEq)]
pub enum BetState{
    Created,
    Started,
    PlayerAWon,
    PlayerBWon,
    Draw
}