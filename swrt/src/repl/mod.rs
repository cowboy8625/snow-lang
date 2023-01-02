// TODO:[6] Make a repl for VM.
//     - Commands
//       1. outputing what you wrote to a file.
//       2. taking a file in and stepping threw it.
//       3. checking all registers values
//       4. checking the heap
//       5. checking the stack
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

pub fn repl() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}
