mod error;
mod eval;
mod expr;
mod parse;
mod value;

pub use error::EvalError;
pub use eval::Resolve;
use parse::parse;
pub use parse::TokenType;
pub use value::Value;

pub fn eval<R>(expr: &str, resolver: &mut R) -> Result<Value, EvalError>
where
    R: Resolve,
{
    let ast = parse(expr)?;
    ast.eval(resolver)
}
