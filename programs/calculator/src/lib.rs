use anchor_lang::prelude::*;

declare_id!("5igcP1nawEo1A31YtZSj9z4nqrnbAvhmcQsoHXccwEDD");

#[program]
pub mod calculator {
    use super::*;
    
    pub fn init(){

    }

    pub fn double(){

    }

    pub fn add(num: u32){
        
    }
}

#[account]
struct DataShape {
    pub num: u32
}

pub struct Initialize<'info>{
    pub account: Account<'info, DataShape>,
    pub system_program: Program<'info, System>,
    signer: Signer<'info>
}