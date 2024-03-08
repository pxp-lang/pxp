use std::{env::args, path::PathBuf, process::exit, time::Instant};

use pxp_indexer::{
    debuggable_entity, try_load_index_from_cache, write_index_to_cache, Index, Indexer,
};
use pxp_symbol::SymbolTable;
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.contains(&"--help".to_string()) {
        println!("Usage: index <directory> [--help] [--no-interaction | -n]");
        exit(0);
    }

    let directory = PathBuf::from(args.first().unwrap());

    println!("Indexing...");
    let start = Instant::now();

    let mut indexer = if let Some(result) = try_load_index_from_cache(&directory) {
        Indexer::of(result.0, result.1)
    } else {
        Indexer::new()
    };

    let (index, symbol_table) = indexer.index(&directory);
    write_index_to_cache((&index, &symbol_table), &directory);

    let duration = start.elapsed();

    println!(
        "Indexing completed. Took {} milliseconds.",
        duration.as_millis()
    );

    if args.contains(&"--no-interaction".to_string()) || args.contains(&"-n".to_string()) {
        return;
    }

    println!();
    println!("Enter a search query below to look through the index.");

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let command = rl.readline(">> ");

        match &command {
            Ok(command) => {
                let _ = rl.add_history_entry(command);

                if command == "clear" {
                    rl.clear_screen().unwrap();
                } else {
                    let split: Vec<&str> = command.split_whitespace().collect();
                    let (command, args) = (split.first().unwrap(), &split[1..]);

                    process_command(command, args, &index, &symbol_table);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting...");
                break;
            }
            _ => unimplemented!(),
        }
    }
}

fn process_command(command: &str, args: &[&str], index: &Index, symbol_table: &SymbolTable) {
    match command {
        "help" => {
            println!("help              Display this help message.");
            println!("search <name>     Search through the index for the given entity (short or full name).");
            println!("dump              Output a minified list of all indexed entities.");
            println!("exit              Exit the REPL.")
        }
        "search" => {
            println!("Searching index for entities matching '{}'", args[0]);

            match symbol_table.find(args[0].as_bytes()) {
                Some(symbol) => {
                    for class_like in index.get_class_likes() {
                        if class_like.name == symbol || class_like.short_name == symbol {
                            println!("{:?}", debuggable_entity(
                                class_like,
                                symbol_table,
                                Box::new(|class, symbol_table, f| {
                                    write!(
                                        f,
                                        "{}{}{}{}",
                                        if class.r#final {
                                            "final "
                                        } else if class.r#abstract {
                                            "abstract "
                                        } else {
                                            ""
                                        },
                                        if class.readonly { " readonly " } else { "" },
                                        if class.is_class {
                                            "class "
                                        } else if class.is_interface {
                                            "interface "
                                        } else if class.is_enum {
                                            "enum "
                                        } else if class.is_trait {
                                            "trait "
                                        } else {
                                            unreachable!()
                                        },
                                        symbol_table.resolve(class.name).unwrap()
                                    )?;

                                    if class.is_enum && class.backing_type.is_valid() {
                                        write!(f, ": {}", class.backing_type)?;
                                    }

                                    if !class.extends.is_empty() {
                                        write!(
                                            f,
                                            "\n        extends {}",
                                            symbol_table
                                                .resolve(*class.extends.first().unwrap())
                                                .unwrap()
                                        )?;
                                    }

                                    if !class.implements.is_empty() {
                                        write!(
                                            f,
                                            "\n        implements {}",
                                            class
                                                .implements
                                                .iter()
                                                .map(|i| format!(
                                                    "{}",
                                                    symbol_table.resolve(*i).unwrap()
                                                ))
                                                .collect::<Vec<String>>()
                                                .join(", ")
                                        )?;
                                    }

                                    if !class.uses.is_empty() {
                                        write!(
                                            f,
                                            "\n        uses {}",
                                            class
                                                .uses
                                                .iter()
                                                .map(|i| format!(
                                                    "{}",
                                                    symbol_table.resolve(*i).unwrap()
                                                ))
                                                .collect::<Vec<String>>()
                                                .join(", ")
                                        )?;
                                    }

                                    if !class.constants.is_empty() {
                                        writeln!(f)?;

                                        for (i, constant) in class.constants.iter().enumerate() {
                                            write!(
                                                f,
                                                "        {}{} const {:?} {}{}",
                                                if constant.r#final { "final " } else { "" },
                                                constant.visibility,
                                                constant.r#type.with_symbol_table(symbol_table),
                                                symbol_table.resolve(constant.name).unwrap(),
                                                if i < class.constants.len() - 1 {
                                                    "\n"
                                                } else {
                                                    ""
                                                }
                                            )?;
                                        }
                                    }

                                    if !class.cases.is_empty() {
                                        writeln!(f)?;

                                        for (i, case) in class.cases.iter().enumerate() {
                                            write!(
                                                f,
                                                "        case {}{}{}",
                                                symbol_table.resolve(*case).unwrap(),
                                                if class.backing_type.is_valid() {
                                                    " = ..."
                                                } else {
                                                    ""
                                                },
                                                if i < class.cases.len() - 1 { "\n" } else { "" }
                                            )?;
                                        }
                                    }

                                    if !class.properties.is_empty() {
                                        writeln!(f)?;

                                        for (i, property) in class.properties.iter().enumerate() {
                                            write!(
                                                f,
                                                "        {} {}{:?} {}{}{}",
                                                property.visibility,
                                                if property.r#static { "static " } else { "" },
                                                property.r#type.with_symbol_table(symbol_table),
                                                symbol_table.resolve(property.name).unwrap(),
                                                if property.default { " = ..." } else { "" },
                                                if i < class.properties.len() - 1 {
                                                    "\n"
                                                } else {
                                                    ""
                                                }
                                            )?;
                                        }
                                    }

                                    if !class.methods.is_empty() {
                                        writeln!(f)?;

                                        for (i, method) in class.methods.iter().enumerate() {
                                            write!(
                                                f,
                                                "        {}{}{} function {}(",
                                                if method.r#final { "final " } else { "" },
                                                method.visibility,
                                                if method.r#static { " static" } else { "" },
                                                symbol_table.resolve(method.name).unwrap()
                                            )?;

                                            for (i, parameter) in
                                                method.parameters.iter().enumerate()
                                            {
                                                write!(
                                                    f,
                                                    "{:?}",
                                                    parameter
                                                        .r#type
                                                        .with_symbol_table(symbol_table)
                                                )?;
                                                write!(f, " ")?;

                                                if parameter.reference {
                                                    write!(f, "&")?;
                                                } else if parameter.variadic {
                                                    write!(f, "...")?;
                                                }

                                                write!(
                                                    f,
                                                    "{}",
                                                    symbol_table.resolve(parameter.name).unwrap()
                                                )?;

                                                if parameter.optional {
                                                    write!(f, " = ...")?;
                                                }

                                                if i < method.parameters.len() - 1 {
                                                    write!(f, ", ")?;
                                                }
                                            }

                                            write!(
                                                f,
                                                "): {:?}",
                                                method.return_type.with_symbol_table(symbol_table)
                                            )?;

                                            if i < class.methods.len() - 1 {
                                                writeln!(f)?;
                                            }
                                        }
                                    }

                                    write!(
                                        f,
                                        "\n        ({} on line {})",
                                        class.location.file, class.location.span.start.line
                                    )?;

                                    Ok(())
                                })
                            ));
                        }
                    }

                    for function in index.get_functions() {
                        if function.name == symbol || function.short_name == symbol {
                            println!("{:?}", debuggable_entity(
                                function,
                                symbol_table,
                                Box::new(|function, symbol_table, f| {
                                    write!(f, "{}(", symbol_table.resolve(function.name).unwrap())?;

                                    for (i, parameter) in function.parameters.iter().enumerate() {
                                        write!(
                                            f,
                                            "{:?}",
                                            debuggable_entity(
                                                parameter,
                                                symbol_table,
                                                Box::new(|parameter, symbol_table, f| {
                                                    write!(
                                                        f,
                                                        "{:?}",
                                                        parameter
                                                            .r#type
                                                            .with_symbol_table(symbol_table)
                                                    )?;
                                                    write!(f, " ")?;

                                                    if parameter.reference {
                                                        write!(f, "&")?;
                                                    } else if parameter.variadic {
                                                        write!(f, "...")?;
                                                    }

                                                    write!(
                                                        f,
                                                        "{}",
                                                        symbol_table.resolve(parameter.name).unwrap()
                                                    )?;

                                                    if parameter.optional {
                                                        write!(f, " = ...")?;
                                                    }

                                                    Ok(())
                                                })
                                            )
                                        )?;

                                        if i < function.parameters.len() - 1 {
                                            write!(f, ", ")?;
                                        }
                                    }

                                    write!(f, "): ")?;
                                    writeln!(
                                        f,
                                        "{:?}",
                                        function.return_type.with_symbol_table(symbol_table)
                                    )?;
                                    write!(
                                        f,
                                        "        ({} on line {})",
                                        function.location.file, function.location.span.start.line
                                    )?;

                                    writeln!(f)
                                })
                            ));
                        }
                    }

                    for constant in index.get_constants() {
                        if constant.name == symbol || constant.short_name == symbol {
                            println!(
                                "    const {}",
                                symbol_table.resolve(constant.name).unwrap()
                            );

                            println!(
                                "\n        ({} on line {})\n",
                                constant.location.file, constant.location.span.start.line
                            );
                        }
                    }
                }
                None => println!("No results found."),
            }
        }
        "dump" => {
            print!("{:?}", index.debuggable(symbol_table));
        }
        "exit" => {
            exit(0);
        }
        _ => {
            println!("Unrecognised command.");
        }
    }
}
