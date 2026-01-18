mod agent;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::Command;


#[derive(Parser)]
#[command(name = "flowforge")]
#[command(about ="Automate developer project setup", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init {
        /// Name of the project
        name: String,

        ///Programming language
        #[arg(long, default_value = "basic")]
        lang: String,
    },
/// Run the AI agent
  Ai {
   /// Ai action (e.g. help)
   action:  Vec <String>,
}
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, lang } => {
            create_project(&name, &lang);
            init_git(&name);
        }
    Commands::Ai { action } => {
       agent::ai::handle_ai(action);
}
      }  
    
   }
fn create_project(name: &str, lang: &str) {
    let project_path = Path::new(name);

    if project_path.exists() {
        eprintln!("project '{}' already exists.", name);
        return;
    }

// Create directories
fs::create_dir_all(format!("{}/src", name))
    .expect("Failed to create project directories");

// Create README
fs::write(
   format!("{}/README.md", name),
   format!("# {}\n\nCreated with Flowforge", name),

)
.expect("Failed to create README");


// Create .gitignore
fs::write(
   format!("{}/.gitignore", name),
   "target/\n.DS_Store\n",
)
.expect("Failed to create .gitignore");

// Create main source file
let main_content = match lang {
   "rust" => r#"

fn main() {
  println!("Hello from your Rust project!");
}

"#,
    _=> r#"
fn main() {
   println!("Hello from your project!")
}
"#, 
   };

  fs::write(format!("{}/src/main.rs", name), main_content)
     .expect("Failed to create main.rs");
  
  println!("project '{}' created successfully!", name);
}

fn init_git(name: &str) {
  let path = Path::new(name);


println!("Initalizing git repository...");

Command::new("git")
   .arg("init")
   .current_dir(path)
   .status()
   .expect("Git init failed");


Command::new("git")
   .args(["add", "."])
   .current_dir(path)
   .status()
   .expect("Git add failed");


Command::new("git")
   .args(["commit", "-m", "Initial commit"])
   .current_dir(path)
   .status()
   .expect("Git commit failed");


println!("Git repository initialized");

}

