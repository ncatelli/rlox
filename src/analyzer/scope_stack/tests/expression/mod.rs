use crate::analyzer::scope_stack::ScopeAnalyzer;
use crate::analyzer::SemanticAnalyzer;
use crate::ast::expression::Expr;

#[test]
fn primary_expression_should_return_ok() {
    let input = Expr::Primary(obj_bool!(true));
    let output = input.clone();

    assert_eq!(Ok(output), ScopeAnalyzer::new().analyze(input));
}

#[test]
fn assignment_expression_should_match_predefined_value() {
    let sa = ScopeAnalyzer::new();
    let input = Expr::Assignment(
        identifier_name!("test"),
        Box::new(Expr::Primary(obj_bool!(true))),
    );
    let output = Expr::Assignment(identifier_id!(0), Box::new(Expr::Primary(obj_bool!(true))));

    // Pre-declare a test variable for the above assignment to assign to
    sa.declare_or_assign(identifier_name!("test"));

    assert_eq!(Ok(output), sa.analyze(input));
}

#[test]
fn assignment_expression_should_err_if_variable_is_undeclared() {
    let input = Expr::Assignment(
        identifier_name!("test"),
        Box::new(Expr::Primary(obj_bool!(true))),
    );

    assert!(ScopeAnalyzer::new().analyze(input).is_err());
}

#[test]
fn variable_analyze_should_resolve_offset() {
    let sa = ScopeAnalyzer::new();
    sa.declare_or_assign(identifier_name!("a"));

    let input = Expr::Variable(identifier_name!("a"));
    let output = Expr::Variable(identifier_id!(0));

    assert_eq!(Ok(output), sa.analyze(input));
}

#[test]
fn call_expression_should_match_predefined_value() {
    let sa = ScopeAnalyzer::new();
    let input = Expr::Assignment(
        identifier_name!("test"),
        Box::new(Expr::Primary(obj_bool!(true))),
    );
    let output = Expr::Assignment(identifier_id!(0), Box::new(Expr::Primary(obj_bool!(true))));

    // Pre-declare a test variable for the above assignment to assign to
    sa.declare_or_assign(identifier_name!("test"));

    assert_eq!(Ok(output), sa.analyze(input));
}

#[test]
fn call_analyze_should_resolve_identifiers_to_ids() {
    let sa = ScopeAnalyzer::new();
    sa.declare_or_assign(identifier_name!("a"));
    sa.declare_or_assign(identifier_name!("b"));

    let input = Expr::Call(
        Box::new(Expr::Variable(identifier_name!("a"))),
        vec![Expr::Variable(identifier_name!("b"))],
    );
    let output = Expr::Call(
        Box::new(Expr::Variable(identifier_id!(0))),
        vec![Expr::Variable(identifier_id!(1))],
    );

    assert_eq!(Ok(output), sa.analyze(input));
}
