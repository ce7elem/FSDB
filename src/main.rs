use std::path::Path;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new table
    Create {
        #[command(subcommand)]
        command: CreateCommands,
    },

    Drop {
        #[command(subcommand)]
        command: DropCommands,
    },
}

#[derive(Subcommand)]
enum CreateCommands {
    /// Create a new table
    Table {
        /// Name of the table
        name: String,
    },
    /// Create a new column in a table
    Column {
        /// Name of the table
        #[arg(long)]
        table: String,
        /// Name of the column
        #[arg(long)]
        name: String,
        /// Type of the column
        #[arg(long)]
        r#type: String,
        /// References for the column (if type is ref[])
        #[arg(long)]
        references: Option<String>,
    },
}

#[derive(Subcommand)]
enum DropCommands {
    /// Create a new table
    Table {
        /// Name of the table
        name: String,
    },
    /// Create a new column in a table
    Column {
        /// Name of the table
        #[arg(long)]
        table: String,
        /// Name of the column
        #[arg(long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { command } => match command {
            CreateCommands::Table { name } => {
                println!("mkdir {name}");
            }
            CreateCommands::Column {
                table,
                name,
                r#type,
                references,
            } => {
                println!("mkdir {table}/{name} (type: {type})");
                match r#type.as_str() {
                    "ref" => {
                        if let Some(reference) = references {
                            assert!(
                                Path::new(reference).is_dir(),
                                "Relation {table}.{name} -> {reference} is invalid: {reference} table does not exist."
                            );
                            println!("echo 'ref {reference}' > {table}/{name}/.type");
                        } else {
                            panic!(
                                "Invalid arguments. Type `ref` is expecting `--reference <table>`"
                            );
                        }
                    }
                    "string" => println!("echo 'string' > {table}/{name}/.type"),
                    _ => panic!("Type not supported"),
                }
            }
        },

        Commands::Drop { command } => match command {
            DropCommands::Table { name } => {
                println!("rm -rf {name}");
            }
            DropCommands::Column { table, name } => {
                println!("rm -rf {table}/{name}");
            }
        },
    }
}
