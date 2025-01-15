use pxp_ast::{Node, NodeId, ResolvedName};
use pxp_diagnostics::Severity;
use pxp_index::{Index, ReflectionFunction};
use pxp_inference::TypeMap;
use pxp_span::Span;
use pxp_type::Type;

use crate::{AnalyserDiagnostic, Reporter};

pub struct AnalyserContext<'a, 'b> {
    reporter: &'a mut Reporter,
    types: TypeMap,
    pub(crate) index: &'b Index,
    file: usize,
    pub(crate) scope: Scope<'b>,
}

pub(crate) struct Scope<'a> {
    pub(crate) function: Option<ReflectionFunction<'a>>,
}

impl<'a, 'b> AnalyserContext<'a, 'b> {
    pub fn new(reporter: &'a mut Reporter, types: TypeMap, index: &'b Index, file: usize) -> Self {
        Self {
            reporter,
            types,
            index,
            file,
            scope: Scope { function: None },
        }
    }

    pub fn report(&mut self, diagnostic: AnalyserDiagnostic, severity: Severity, span: Span) {
        self.reporter.report(self.file, diagnostic, severity, span);
    }

    pub fn get_type_of_node(&self, node: &Node) -> &Type<ResolvedName> {
        self.get_type(node.id)
    }

    pub fn get_type(&self, id: NodeId) -> &Type<ResolvedName> {
        self.types.resolve(id)
    }

    pub fn get_function(&self) -> Option<&ReflectionFunction> {
        self.scope.function.as_ref()
    }
}
