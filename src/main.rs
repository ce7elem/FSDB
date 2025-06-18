use std::path::Path;

use clap::{Args, Parser, Subcommand};

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

    Select(SelectCommands),
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
    /// Drop a new table
    Table {
        /// Name of the table
        name: String,
    },
    /// Drop a new column in a table
    Column {
        /// Name of the table
        #[arg(long)]
        table: String,
        /// Name of the column
        #[arg(long)]
        name: String,
    },
}

#[derive(Debug, Args)]
struct SelectCommands {
    /// eg. user.name
    #[arg(long = "column")]
    columns: Vec<String>,
    /// eg. user.name == "naruto"
    #[arg(long = "filter")]
    filters: Vec<String>,
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
                    coltype => println!("echo {coltype} > {table}/.{name}"),
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

        Commands::Select(SelectCommands { filters, columns }) => {
            if filters.len() > 1 {
                unimplemented!("We support only one filter for now");
            }

            let rows = if filters.is_empty() {
                vec!["*"]
            } else {
                let filtered_rows = vec![];

                for filter in filters {
                    let [ressource, value] = filter.split("=").collect::<Vec<_>>()[..] else {
                        unimplemented!(
                            "Only equality is supported. Expected <table>.<colname> = <match>"
                        );
                    };
                    let [table, column] = ressource.split(".").collect::<Vec<_>>()[..] else {
                        panic!("Expected <table>.<colname>");
                    };

                    println!(
                        "grep -rI {value} `find tmp/ -type f -wholename '*/{table}/*/{column}'`"
                    );
                    // push restult into filtered_rows
                }

                filtered_rows
            };

            for ressource in columns {
                let [table, column] = ressource.split(".").collect::<Vec<_>>()[..] else {
                    panic!("Expected <table>.<colname>");
                };
                println!(
                    "cat {table}/{{{rows_ids}}}/{column}",
                    rows_ids = rows.join(",")
                );
            }
        }
    }
}
