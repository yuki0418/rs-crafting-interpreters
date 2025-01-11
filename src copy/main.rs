use rs_crafting_interpreters::rlox::RLox;

fn main() {
    let mut rlox = RLox::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        rlox.run_file(args[1].clone());
    } else {
        rlox.run_prompt();
    }
}
