use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EditionCouter{
    pub edition_count:u32,
    
}