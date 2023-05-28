use pest::pratt_parser::PrattParser;

use crate::Rule;
use pest::pratt_parser::{Assoc::*, Op};

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        PrattParser::new()
            .op(Op::infix(Rule::comma, Left))
            .op(Op::infix(Rule::assign, Right))
            .op(Op::infix(Rule::or, Left))
            .op(Op::infix(Rule::and, Left))
            .op(Op::infix(Rule::bit_or, Left))
            .op(Op::infix(Rule::bit_xor, Left))
            .op(Op::infix(Rule::bit_and, Left))
            .op(Op::infix(Rule::eq, Left)
                | Op::infix(Rule::neq, Left))

            .op(Op::infix(Rule::greater, Left)
                | Op::infix(Rule::lesser, Left)
                | Op::infix(Rule::greater_eq, Left)
                | Op::infix(Rule::lesser_eq, Left))

            .op(Op::infix(Rule::add, Left)
                | Op::infix(Rule::subtract, Left))

            .op(Op::infix(Rule::multiply, Left)
                | Op::infix(Rule::divide, Left))

            .op(Op::infix(Rule::pow, Right))
            .op(Op::prefix(Rule::inc)
                | Op::prefix(Rule::dec)
                | Op::prefix(Rule::plus)
                | Op::prefix(Rule::minus)
                | Op::prefix(Rule::not)
                | Op::prefix(Rule::bit_not))

            .op(Op::postfix(Rule::post_inc)
                | Op::postfix(Rule::post_dec)
                | Op::infix(Rule::field_access, Left))

    };
}
