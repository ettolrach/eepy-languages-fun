enum Expr {
    Num(u32),
    Plus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    Bool(bool),
    Eq(Box<Expr>, Box<Expr>),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
}

enum Value {
    NumV(u32),
    BoolV(bool),
}

fn add(v1: Value, v2: Value) -> Option<Value> {
    match (v1, v2) {
        (Value::NumV(n1), Value::NumV(n2)) => Some(Value::NumV(n1 + n2)),
        (_, _) => None,
    }
}

fn mult(v1: Value, v2: Value) -> Option<Value> {
    match (v1, v2) {
        (Value::NumV(n1), Value::NumV(n2)) => Some(Value::NumV(n1 * n2)),
        (_, _) => None,
    }
}

fn eq(v1: Value, v2: Value) -> Option<Value> {
    match (v1, v2) {
        (Value::NumV(n1), Value::NumV(n2)) => Some(Value::BoolV(n1 == n2)),
        (Value::BoolV(b1), Value::BoolV(b2)) => Some(Value::BoolV(b1 == b2)),
        _ => None,
    }
}

fn size(e: Expr) -> u32 {
    match e {
        Expr::Num(_) => 1,
        Expr::Plus(e1, e2) => size(*e1) + size(*e2) + 1,
        Expr::Times(e1, e2) => size(*e1) * size(*e2) + 1,
        _ => unimplemented!(),
    }
}

fn eval(e: Expr) -> Option<Value> {
    match e {
        Expr::Num(n) => Some(Value::NumV(n)),
        Expr::Plus(e1, e2) => add(eval(*e1)?, eval(*e2)?),
        Expr::Times(e1, e2) => mult(eval(*e1)?, eval(*e2)?),
        Expr::Bool(b) => Some(Value::BoolV(b)),
        Expr::Eq(e1, e2) => eq(eval(*e1)?, eval(*e2)?),
        Expr::IfThenElse(e, e1, e2) => {
            match eval(*e) {
                Some(Value::BoolV(true)) => eval(*e1),
                Some(Value::BoolV(false)) => eval(*e2),
                _ => None,
            }
        },
    }
}

fn main() {
    let expression: Expr = Expr::Plus(Box::new(Expr::Num(1)), Box::new(Expr::Times(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)))));
    let res = eval(expression);
    match res {
        Some(Value::NumV(n)) => {
            assert_eq!(7, n);
            println!("No type error! :)")
        },
        _ => println!("Type error :(")
    }
}
