use nexum_core::{StorageEngine, Parser, Executor};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    println!("NexumDB v0.1.0 - AI-Native Database");
    println!("Initializing...\n");

    let storage = StorageEngine::new("./nexumdb_data")?;
    let executor = Executor::new(storage).with_cache();
    
    println!("Ready. Type SQL commands or 'exit' to quit\n");

    loop {
        print!("nexumdb> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        match Parser::parse(input) {
            Ok(statement) => {
                match executor.execute(statement) {
                    Ok(result) => {
                        println!("{:?}", result);
                    }
                    Err(e) => {
                        eprintln!("Execution error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
            }
        }
    }

    Ok(())
}
