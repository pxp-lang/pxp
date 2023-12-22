use std::collections::HashMap;

use pxp_symbol::{Symbol, SymbolTable};

use crate::{entities::FunctionEntity, DebuggableEntityWithSymbolTable, debuggable_entity};

#[derive(Debug, Clone, Default)]
pub struct Index {
    // Using Symbol as the key for entities is a good idea because it
    // allows us to do super fast lookups when we have a resolved identifier.
    functions: HashMap<Symbol, FunctionEntity>,
}

impl Index {
    pub fn debuggable<'a>(&'a self, symbol_table: &'a SymbolTable) -> DebuggableEntityWithSymbolTable<Self> {
        debuggable_entity(self.clone(), symbol_table, Box::new(|index, symbol_table, f| {
            writeln!(f, "Functions ({}):", index.get_number_of_functions())?;

            for function in index.get_functions() {
                write!(f, "    {:?}", debuggable_entity(function, symbol_table, Box::new(|function, symbol_table, f| {
                    write!(f, "{}(", symbol_table.resolve(function.name).unwrap())?;

                    for (i, parameter) in function.parameters.iter().enumerate() {
                        write!(f, "{:?}", debuggable_entity(parameter, symbol_table, Box::new(|parameter, symbol_table, f| {
                            write!(f, "{:?}", parameter.r#type.with_symbol_table(symbol_table))?;
                            write!(f, " ")?;

                            if parameter.reference {
                                write!(f, "&")?;
                            } else if parameter.variadic {
                                write!(f, "...")?;
                            }

                            write!(f, "{}", symbol_table.resolve(parameter.name).unwrap())?;

                            if parameter.optional {
                                write!(f, " = ...")?;
                            }

                            Ok(())
                        })))?;

                        if i < function.parameters.len() - 1 {
                            write!(f, ", ")?;
                        }
                    }

                    write!(f, "): ")?;
                    writeln!(f, "{:?}", function.return_type.with_symbol_table(symbol_table))?;
                    write!(f, "        Location: {} on line {}", function.location.file, function.location.span.start.line)?;

                    writeln!(f)
                })))?;
            }

            writeln!(f, "")
        }))
    }
}

impl Index {
    pub fn add_function(&mut self, function: FunctionEntity) {
        self.functions.insert(function.name, function);
    }

    pub fn get_functions(&self) -> impl Iterator<Item = &FunctionEntity> {
        self.functions.values()
    }

    pub fn get_number_of_functions(&self) -> usize {
        self.functions.len()
    }
}
