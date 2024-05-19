use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use syn::ExprParen;

pub fn handle_paren_expression(
    expr_paren: &ExprParen,
    _assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    println!("\n[DEBUG]-[Paren]: handle_paren_expression");

    Ok(parse_expression(&expr_paren.expr, None)?)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use syn::ExprParen;

//     #[test]
//     fn test_paren_expression() {
//         let parsed_expr_paren: ExprParen = syn::parse_str("(1)").unwrap();
//         let result = parse_expression(&syn::Expr::Paren(parsed_expr_paren), None);
//         let expected: Vec<Instruction> = vec![Instruction::new(
//             "assign".to_string(),
//             vec!["1".to_string()],
//             "RETURN_VALUE_LABEL".to_string(),
//         )];
//     }
// }