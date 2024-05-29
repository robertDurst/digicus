// use super::pattern::handle_pattern;
use crate::errors::not_translatable_error::NotTranslatableError;
use crate::instruction::Instruction;
use crate::translate::expression::parse_expression;
use crate::translate::pattern::handle_pattern;
use crate::translate::statement::macro_statement::handle_macro_statement;
use syn::ExprBlock;

// A block is a collection of statements
pub fn handle_block_expression(
    _expr_block: &ExprBlock,
    _assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    // let the_block = expr_block.block.clone();
    // let mut block_str: Vec<String> = Vec::new();

    // for stmt in the_block.stmts.iter() {
    // let stmt_str = parse_block_stmt(stmt)?;
    // block_str.push_str(&stmt_str);

    //     match &parse_block_stmt(stmt) {
    //         Ok(stmt_str) => block_str.push(stmt_str.to_string()),
    //         Err(e) => return Err(e.clone()),
    //     }
    // }

    // Ok(block_str.join("\n"))

    Ok(vec![Instruction::new(
        "block".to_string(),
        vec![],
        "".to_string(),
    )])
}

pub fn parse_block_stmt(
    stmt: &syn::Stmt,
    assignment: Option<String>,
) -> Result<Vec<Instruction>, NotTranslatableError> {
    match stmt {
        syn::Stmt::Local(local) => {
            let pattern_as_string = handle_pattern(local.pat.clone()).unwrap();
            match &local.init {
                Some(local_init) => parse_expression(&local_init.expr, Some(pattern_as_string)),
                None => Ok(vec![Instruction::new(
                    "assign".to_string(),
                    vec![pattern_as_string],
                    "".to_string(),
                )]),
            }
        }
        syn::Stmt::Item(_item) => Err(NotTranslatableError::Custom(
            "Item statement not translatable".to_string(),
        )),
        syn::Stmt::Expr(exp, _r) => parse_expression(exp, assignment),
        syn::Stmt::Macro(stmt_mac) => handle_macro_statement(stmt_mac, assignment),
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::translate::expression::parse_expression;
//     use syn::ExprBlock;

//     #[test]
//     fn test_block_expression() {
//         let parsed_expr_block: ExprBlock = syn::parse_str("{ let x = 1; let foo = bar; }").unwrap();
//         let result = parse_expression(&syn::Expr::Block(parsed_expr_block), None);

//         let expected = "{ instruction: assign, input: (1), assign: x }
// { instruction: assign, input: (bar), assign: foo }";

//         assert_eq!(result, Ok(expected.to_string()));
//     }
// }
