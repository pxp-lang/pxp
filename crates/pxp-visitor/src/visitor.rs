use pxp_ast::{Statement, FullOpeningTagStatement, ShortOpeningTagStatement, EchoOpeningTagStatement, ClosingTagStatement, InlineHtmlStatement, goto::{LabelStatement, GotoStatement}, StaticStatement, GlobalStatement, HaltCompilerStatement, loops::{DoWhileStatement, WhileStatement, ForStatement, ForeachStatement, BreakStatement, ContinueStatement}, constant::ConstantStatement, functions::FunctionStatement, classes::ClassStatement, traits::TraitStatement, interfaces::InterfaceStatement, control_flow::IfStatement, SwitchStatement, EchoStatement, ExpressionStatement, ReturnStatement, namespaces::NamespaceStatement, UseStatement, GroupUseStatement, try_block::TryStatement, enums::{UnitEnumStatement, BackedEnumStatement}, BlockStatement, declares::DeclareStatement};
use pxp_span::Span;
use pxp_syntax::comments::Comment;
use crate::walk::*;

pub trait Visitor {
    fn visit(&mut self, program: &mut [Statement]) {
        walk(self, program);
    }

    fn visit_statement(&mut self, statement: &mut Statement) {
        walk_statement(self, statement)
    }

    fn visit_full_opening_tag(&mut self, statement: &mut FullOpeningTagStatement) {}
    fn visit_short_opening_tag(&mut self, statement: &mut ShortOpeningTagStatement) {}
    fn visit_echo_opening_tag(&mut self, statement: &mut EchoOpeningTagStatement) {}
    fn visit_closing_tag(&mut self, statement: &mut ClosingTagStatement) {}

    fn visit_inline_html(&mut self, statement: &mut InlineHtmlStatement) {}
    fn visit_halt_compiler(&mut self, statement: &mut HaltCompilerStatement) {}

    fn visit_label(&mut self, statement: &mut LabelStatement) {}
    fn visit_goto(&mut self, statement: &mut GotoStatement) {}

    fn visit_static(&mut self, statement: &mut StaticStatement) {}
    fn visit_global(&mut self, statement: &mut GlobalStatement) {}

    fn visit_do_while(&mut self, statement: &mut DoWhileStatement) {
        todo!("walk do while")
    }

    fn visit_while(&mut self, statement: &mut WhileStatement) {
        todo!("walk while")
    }

    fn visit_for(&mut self, statement: &mut ForStatement) {
        todo!("walk for")
    }

    fn visit_foreach(&mut self, statement: &mut ForeachStatement) {
        todo!("walk foreach")
    }

    fn visit_if(&mut self, statement: &mut IfStatement) {
        todo!("walk if")
    }

    fn visit_switch(&mut self, statement: &mut SwitchStatement) {
        todo!("walk switch")
    }

    fn visit_break(&mut self, statement: &mut BreakStatement) {
        todo!("walk break")
    }

    fn visit_continue(&mut self, statement: &mut ContinueStatement) {
        todo!("walk continue")
    }

    fn visit_constant(&mut self, statement: &mut ConstantStatement) {
        todo!("walk constant")
    }

    fn visit_function(&mut self, statement: &mut FunctionStatement) {
        todo!("walk function")
    }

    fn visit_class(&mut self, statement: &mut ClassStatement) {
        todo!("walk class")
    }

    fn visit_trait(&mut self, statement: &mut TraitStatement) {
        todo!("walk trait")
    }

    fn visit_interface(&mut self, statement: &mut InterfaceStatement) {
        todo!("walk interface")
    }

    fn visit_echo(&mut self, statement: &mut EchoStatement) {
        todo!("walk echo")
    }

    fn visit_expression_stmt(&mut self, statement: &mut ExpressionStatement) {
        todo!("walk expression")
    }

    fn visit_return(&mut self, statement: &mut ReturnStatement) {
        todo!("walk return")
    }

    fn visit_namespace(&mut self, statement: &mut NamespaceStatement) {
        todo!("walk namespace")
    }

    fn visit_use(&mut self, statement: &mut UseStatement) {
        todo!("walk use")
    }

    fn visit_group_use(&mut self, statement: &mut GroupUseStatement) {
        todo!("walk group use")
    }

    fn visit_comment_stmt(&mut self, statement: &mut Comment) {
        todo!("walk comment")
    }

    fn visit_try(&mut self, statement: &mut TryStatement) {
        todo!("walk try")
    }

    fn visit_unit_enum(&mut self, statement: &mut UnitEnumStatement) {
        todo!("walk unit enum")
    }

    fn visit_backed_enum(&mut self, statement: &mut BackedEnumStatement) {
        todo!("walk backed enum")
    }

    fn visit_block(&mut self, statement: &mut BlockStatement) {
        todo!("walk block")
    }

    fn visit_declare(&mut self, statement: &mut DeclareStatement) {
        todo!("walk declare")
    }

    fn visit_noop(&mut self, statement: Span) {
        todo!("walk noop")
    }
}