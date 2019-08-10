#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(test)]

extern crate test;

use std::fs;
use pretty_assertions::assert_eq;

use tf_demo_parser::{Demo, DemoParser, MatchState, MessageTypeAnalyser, MessageType, ParserState};
use tf_demo_parser::demo::packet::datatable::{SendTable, SendTableName};
use tf_demo_parser::demo::packet::stringtable::StringTableEntry;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::sendprop::SendPropDefinition;
use std::collections::{HashMap, HashSet};
use tf_demo_parser::demo::parser::MessageHandler;
use test::Bencher;

pub struct SendPropAnalyser;

impl MessageHandler for SendPropAnalyser {
    type Output = Vec<SendTable>;

    fn does_handle(message_type: MessageType) -> bool {
        false
    }

    fn handle_message(&mut self, message: Message, tick: u32) {}

    fn handle_string_entry(&mut self, table: &String, _index: usize, entry: &StringTableEntry) {}

    fn get_output(self, state: ParserState) -> Self::Output {
        state.send_tables.into_iter().map(|(_k, v)| v).collect()
    }
}

fn flatten_bench(input_file: &str, b: &mut Bencher) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let stream = demo.get_stream();
    let (_, send_tables) = DemoParser::parse_with_analyser(stream.clone(), SendPropAnalyser).unwrap();
    b.iter(|| {
        let flat: Vec<_> = send_tables.iter().map(|table| table.flatten_props(&send_tables)).collect();
        test::black_box(flat);
    });
}

#[bench]
fn sendprop_test_gully(b: &mut Bencher) {
    flatten_bench("data/gully.dem", b);
}
