use std::collections::HashMap;

use pxp_ast::NodeId;
use pxp_bytestring::ByteString;
use pxp_index::{Index, ReflectionClass, ReflectionFunction};
use pxp_type::Type;

pub type ScopeId = u16;

#[derive(Debug, Clone)]
pub struct Scope {
    pub id: ScopeId,
    pub(crate) class: Option<ByteString>,
    pub(crate) function: Option<ByteString>,
    pub(crate) variables: HashMap<ByteString, Type<ByteString>>,
    pub(crate) types: HashMap<NodeId, Type<ByteString>>,
}

impl Scope {
    pub(crate) fn new(id: ScopeId, class: Option<ByteString>, function: Option<ByteString>) -> Self {
        Self {
            id,
            class,
            function,
            variables: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub(crate) fn get_type(&self, id: NodeId) -> Option<&Type<ByteString>> {
        self.types.get(&id)
    }

    pub(crate) fn insert_type(&mut self, id: NodeId, ty: Type<ByteString>) {
        self.types.insert(id, ty);
    }

    pub(crate) fn insert_variable(&mut self, variable: ByteString, ty: Type<ByteString>) {
        self.variables.insert(variable, ty);
    }

    pub(crate) fn get_variable(&self, variable: &ByteString) -> &Type<ByteString> {
        self.variables.get(variable).unwrap_or_else(|| &Type::Mixed)
    }

    pub fn is_in_class(&self) -> bool {
        self.class.is_some()
    }

    pub fn is_in_function(&self) -> bool {
        self.function.is_some()
    }

    pub fn get_class<'i>(&self, index: &'i Index) -> Option<ReflectionClass<'i>> {
        if let Some(class) = &self.class {
            index.get_class(class)
        } else {
            None
        }
    }

    pub fn get_function<'i>(&self, index: &'i Index) -> Option<ReflectionFunction<'i>> {
        if let Some(func) = &self.function {
            index.get_function(func)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ScopeStack {
    i: usize,
    stack: Vec<Scope>,
}

impl ScopeStack {
    pub(crate) fn new() -> Self {
        ScopeStack {
            i: 0,
            stack: vec![],
        }
    }

    pub(crate) fn push(&mut self) {
        if ! self.stack.is_empty() {
            self.i += 1;
        }

        self.stack.push(Scope::new(self.stack.len() as u16, None, None));
    }

    pub(crate) fn push_inherited(&mut self) {
        let (class, function) = if !self.stack.is_empty() {
            let previous = &self.stack[self.i];
            
            self.i += 1;

            (previous.class.clone(), previous.function.clone())
        } else {
            (None, None)
        };

        self.stack.push(Scope::new(self.stack.len() as u16, class, function));
    }

    pub(crate) fn pop(&mut self) {
        self.i -= 1;
    }

    pub(crate) fn scope(&self) -> &Scope {
        &self.stack[self.i]
    }

    pub(crate) fn scope_mut(&mut self) -> &mut Scope {
        &mut self.stack[self.i]
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<Scope> {
        self.stack.iter()
    }
}