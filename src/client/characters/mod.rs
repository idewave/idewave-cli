use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub mod request_characters;
mod parse_characters;
mod player_login;
pub mod types;

use crate::client::opcodes::Opcode;
use crate::traits::Processor;
use crate::types::{HandlerFunction, HandlerInput, ProcessorResult};

pub struct CharactersProcessor;

impl Processor for CharactersProcessor {
    fn process_input(input: HandlerInput) -> ProcessorResult {
        let mut reader = Cursor::new(input.data.as_ref().unwrap()[2..].to_vec());
        let opcode = reader.read_u16::<LittleEndian>().unwrap();

        let handlers: Vec<HandlerFunction> = match opcode {
            Opcode::SMSG_CHAR_ENUM => {
                println!("RECEIVE SMSG_CHAR_ENUM");
                vec![
                    Box::new(parse_characters::handler),
                    Box::new(player_login::handler),
                ]
            },
            _ => vec![],
        };

        Self::collect_responses(handlers, input)
    }
}