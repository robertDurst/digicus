use syn::ExprAsync;

use crate::errors::not_translatable_error::NotTranslatableError;

pub fn handle_async_expression(_: &ExprAsync) -> Result<String, NotTranslatableError> {
    Err(NotTranslatableError::Custom(
        "Async expression not translatable".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::translate::expression::parse_expression;
    use syn::ExprAsync;

    use super::*;

    #[test]
    fn test_async_expression() {
        let parsed_expr_async: ExprAsync = syn::parse_str("async { }").unwrap();
        let result = parse_expression(&syn::Expr::Async(parsed_expr_async));

        assert_eq!(
            result,
            Err(NotTranslatableError::Custom(
                "Async expression not translatable".to_string()
            ))
        );
    }
}
