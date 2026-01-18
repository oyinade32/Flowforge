/// AI agent module
/// Handles AI-style commands such as explaining project files


use std::fs;
/// Handles AI subcommands
/// Example: ai explain

pub fn handle_ai(args: Vec<String>) {
    if args.is_empty() {
       print_help();
       return;
}

match args[0].as_str() {
   "help" => {
      print_help();

}

 "ask" => {
    if args.len() < 2 {
       println!("Please provide a question.");
       return;
}

let question = args[1..].join(" ");
println!("AI received your question:");
println!("{}", question);
}

"explain" => {
   explain_readme();
}

_=> {
   println!("Unknown AI command.");
   print_help();
     }
  }
}

fn print_help() {
   println!("Flowforge AI Agent");
   println!("Commands:");
   println!("  flowforge ai help");
   println!("  flowforge ai ask <question>");
   println!("  flowforge ai explain");
}

fn explain_readme() {
   let path = "README.md";

   match fs::read_to_string(path) {
        Ok(content) => {
            println!("README.md content:\n");
            println!("{}", content);

            println!("\n AI Explanation:");
            println!("This README describes your project.");
            println!("It usually explains:");
            println!(". What the project does");
            println!(". How to use it");
            println!(". Why it exists");
            println!("This helps other developers understand your work.");
}

Err(_) => {
   println!("README.md not found in this directory.");
     }
  }
}

