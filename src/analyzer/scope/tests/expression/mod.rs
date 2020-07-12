use crate::analyzer::scope::ScopeAnalyzer;
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
