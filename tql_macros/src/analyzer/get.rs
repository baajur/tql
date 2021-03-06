/*
 * Copyright (c) 2017-2018 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/// Analyzer for the get() method.

use syn::Expr;

use ast::{
    Expression,
    Filter,
    FilterExpression,
    FilterValue,
    Limit,
    RelationalOperator,
};
use error::{Result, res};
use plugin::number_literal;
use super::filter::expression_to_filter_expression;

/// Convert an expression from a `get()` method to a FilterExpression and a Limit.
pub fn get_expression_to_filter_expression(arg: &Expression, table_name: &str) ->
    Result<(FilterExpression, bool, Limit)>
{
    match *arg {
        Expr::Lit(_) | Expr::Path(_) => {
            let filter = FilterExpression::Filter(Filter {
                operand1: FilterValue::PrimaryKey(table_name.to_string()),
                operator: RelationalOperator::Equal,
                operand2: arg.clone(),
            });
            res((filter, true, Limit::NoLimit), vec![])
        },
        _ => expression_to_filter_expression(arg, table_name)
            .and_then(|filter| Ok((filter, false, Limit::Index(number_literal(0))))),
    }
}
