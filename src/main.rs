#[macro_use]
extern crate combine;
use combine::parser::char::{char, letter, spaces};
use combine::stream::Stream;
use combine::{between, sep_by, Parser, many1};

#[derive(Debug, PartialEq)]
pub enum Expr {
  Id(String),
  Array(Vec<Expr>),
  Pair(Box<Expr>, Box<Expr>),
}

// The `parser!` macro can be used to define parser producing functions in most cases
// (for more advanced uses standalone functions or explicit implementation of `Parser`
// can be done to handle parsing)
parser!{
   fn expr[I]()(I) -> Expr
    where [I: Stream<Item=char>]
{
    let word = many1(letter());

    //Creates a parser which parses a char and skips any trailing whitespace
    let lex_char = |c| char(c).skip(spaces());

    let comma_list = sep_by(expr(), lex_char(','));
    let array = between(lex_char('['), lex_char(']'), comma_list);

    //We can use tuples to run several parsers in sequence
    //The resulting type is a tuple containing each parsers output
    let pair = (lex_char('('),
                expr(),
                lex_char(','),
                expr(),
                lex_char(')'))
                   .map(|t| Expr::Pair(Box::new(t.1), Box::new(t.3)));

    word.map(Expr::Id)
        .or(array.map(Expr::Array))
        .or(pair)
        .skip(spaces())
}
}

fn main() {
  let result = expr().parse("[[], (hello, world), [rust]]");
  let expr = Expr::Array(vec![
    Expr::Array(Vec::new()),
    Expr::Pair(
      Box::new(Expr::Id("hello".to_string())),
      Box::new(Expr::Id("world".to_string())),
    ),
    Expr::Array(vec![Expr::Id("rust".to_string())]),
  ]);
  assert_eq!(result, Ok((expr, "")));
}
