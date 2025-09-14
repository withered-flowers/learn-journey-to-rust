mod x01_variables;
mod x02_numbers;
mod x03_char_bool_unit;
mod x04_statements_expressions;
mod x05_functions;
mod x06_ownership;
mod x07_borrowing;

fn main() {
    x01_variables::invoker();
    x02_numbers::invoker();
    x03_char_bool_unit::invoker();
    x04_statements_expressions::invoker();
    x05_functions::invoker();
    x06_ownership::invoker();
    x07_borrowing::invoker();
}
