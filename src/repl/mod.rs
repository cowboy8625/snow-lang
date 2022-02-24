mod shell;
use crate::interpreter;
use crate::parser;

pub fn run() {
    shell::run().expect("   Shell Failed.    ");
}
