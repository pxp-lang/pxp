use pxp_ast::{Statement, StatementKind};

use crate::Visitor;

pub fn walk<V: Visitor + ?Sized>(visitor: &mut V, program: &mut [Statement]) {
    for statement in program.iter_mut() {
        visitor.visit_statement(statement);
    }
}

pub fn walk_statement<V: Visitor + ?Sized>(visitor: &mut V, statement: &mut Statement) {
    match &mut statement.kind {
        StatementKind::FullOpeningTag(stmt) => visitor.visit_full_opening_tag(stmt),
        StatementKind::ShortOpeningTag(stmt) => visitor.visit_short_opening_tag(stmt),
        StatementKind::EchoOpeningTag(stmt) => visitor.visit_echo_opening_tag(stmt),
        StatementKind::ClosingTag(stmt) => visitor.visit_closing_tag(stmt),
        StatementKind::InlineHtml(stmt) => visitor.visit_inline_html(stmt),
        StatementKind::Label(stmt) => visitor.visit_label(stmt),
        StatementKind::Goto(stmt) => visitor.visit_goto(stmt),
        StatementKind::HaltCompiler(stmt) => visitor.visit_halt_compiler(stmt),
        StatementKind::Static(stmt) => visitor.visit_static(stmt),
        StatementKind::DoWhile(stmt) => visitor.visit_do_while(stmt),
        StatementKind::While(stmt) => visitor.visit_while(stmt),
        StatementKind::For(stmt) => visitor.visit_for(stmt),
        StatementKind::Foreach(stmt) => visitor.visit_foreach(stmt),
        StatementKind::Break(stmt) => visitor.visit_break(stmt),
        StatementKind::Continue(stmt) => visitor.visit_continue(stmt),
        StatementKind::Constant(stmt) => visitor.visit_constant(stmt),
        StatementKind::Function(stmt) => visitor.visit_function(stmt),
        StatementKind::Class(stmt) => visitor.visit_class(stmt),
        StatementKind::Trait(stmt) => visitor.visit_trait(stmt),
        StatementKind::Interface(stmt) => visitor.visit_interface(stmt),
        StatementKind::If(stmt) => visitor.visit_if(stmt),
        StatementKind::Switch(stmt) => visitor.visit_switch(stmt),
        StatementKind::Echo(stmt) => visitor.visit_echo(stmt),
        StatementKind::Expression(stmt) => visitor.visit_expression_stmt(stmt),
        StatementKind::Return(stmt) => visitor.visit_return(stmt),
        StatementKind::Namespace(stmt) => visitor.visit_namespace(stmt),
        StatementKind::Use(stmt) => visitor.visit_use(stmt),
        StatementKind::GroupUse(stmt) => visitor.visit_group_use(stmt),
        StatementKind::Comment(stmt) => visitor.visit_comment_stmt(stmt),
        StatementKind::Try(stmt) => visitor.visit_try(stmt),
        StatementKind::UnitEnum(stmt) => visitor.visit_unit_enum(stmt),
        StatementKind::BackedEnum(stmt) => visitor.visit_backed_enum(stmt),
        StatementKind::Block(stmt) => visitor.visit_block(stmt),
        StatementKind::Global(stmt) => visitor.visit_global(stmt),
        StatementKind::Declare(stmt) => visitor.visit_declare(stmt),
        StatementKind::Noop(span) => visitor.visit_noop(*span),
    };
}