use std::collections::HashMap;

use pxp_bytestring::ByteStr;
use pxp_symbol::{Symbol, SymbolTable};

use crate::{entities::FunctionEntity, DebuggableEntityWithSymbolTable, debuggable_entity, ClassLikeEntity};

#[derive(Debug, Clone, Default)]
pub struct Index {
    // Using Symbol as the key for entities is a good idea because it
    // allows us to do super fast lookups when we have a resolved identifier.
    functions: HashMap<Symbol, FunctionEntity>,
    class_likes: HashMap<Symbol, ClassLikeEntity>,
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
                    write!(f, "        ({} on line {})", function.location.file, function.location.span.start.line)?;

                    writeln!(f)
                })))?;
            }

            writeln!(f, "Classish ({}):", index.get_number_of_class_likes())?;

            for class in index.get_class_likes() {
                writeln!(f, "    {:?}", debuggable_entity(class, symbol_table, Box::new(|class, symbol_table, f| {
                    write!(f, "{}{}{}", if class.r#final { "final " } else if class.r#abstract { "abstract " } else { "" }, if class.readonly { " readonly " } else { "" }, symbol_table.resolve(class.name).unwrap())?;

                    if !class.extends.is_empty() {
                        write!(f, "\n        extends {}", symbol_table.resolve(*class.extends.get(0).unwrap()).unwrap())?;
                    }

                    if !class.implements.is_empty() {
                        write!(f, "\n        implements {}", class.implements.iter().map(|i| format!("{}", symbol_table.resolve(*i).unwrap())).collect::<Vec<String>>().join(", "))?;
                    }

                    if !class.constants.is_empty() {
                        writeln!(f)?;

                        for (i, constant) in class.constants.iter().enumerate() {
                            write!(f, "        {}{} const {:?} {}{}", if constant.r#final { "final " } else { "" }, constant.visibility, constant.r#type.with_symbol_table(symbol_table), symbol_table.resolve(constant.name).unwrap(), if i < class.constants.len() - 1 { "\n" } else { "" })?;
                        }
                    }

                    if !class.properties.is_empty() {
                        writeln!(f)?;

                        for (i, property) in class.properties.iter().enumerate() {
                            write!(f, "        {} {}{:?} {}{}{}", property.visibility, if property.r#static { "static " } else { "" }, property.r#type.with_symbol_table(symbol_table), symbol_table.resolve(property.name).unwrap(), if property.default { " = ..." } else { "" }, if i < class.properties.len() - 1 { "\n" } else { "" })?;
                        }
                    }

                    if !class.methods.is_empty() {
                        writeln!(f)?;

                        for (i, method) in class.methods.iter().enumerate() {
                            write!(f, "        {}{}{} function {}(", if method.r#final { "final " } else { "" }, method.visibility, if method.r#static { " static" } else { "" }, symbol_table.resolve(method.name).unwrap())?;

                            for (i, parameter) in method.parameters.iter().enumerate() {
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

                                if i < method.parameters.len() - 1 {
                                    write!(f, ", ")?;
                                }
                            }

                            write!(f, "): {:?}", method.return_type.with_symbol_table(symbol_table))?;

                            if i < class.methods.len() - 1 {
                                write!(f, "\n")?;
                            }
                        }
                    }

                    write!(f, "\n        ({} on line {})", class.location.file, class.location.span.start.line)?;

                    Ok(())
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

    pub fn add_class_like(&mut self, class_like: ClassLikeEntity) {
        self.class_likes.insert(class_like.name, class_like);
    }

    pub fn get_class_likes(&self) -> impl Iterator<Item = &ClassLikeEntity> {
        self.class_likes.values()
    }

    pub fn get_number_of_class_likes(&self) -> usize {
        self.class_likes.len()
    }

    pub fn get_classes(&self) -> impl Iterator<Item = &ClassLikeEntity> {
        self.class_likes.values().filter(|class_like| class_like.is_class)
    }

    pub fn get_number_of_classes(&self) -> usize {
        self.class_likes.values().filter(|class_like| class_like.is_class).count()
    }
}
