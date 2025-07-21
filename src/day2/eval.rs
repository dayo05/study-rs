#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// 트리 형식의 표현식입니다.
#[derive(Debug)]
pub enum Expression {
    /// 두 개의 하위 표현식에 관한 연산입니다.
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// 리터럴 값
    Value(i64),
}

pub fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op {
            op: Operation::Add,
            left,
            right,
        } => Ok(eval(*left)? + eval(*right)?),
        Expression::Op {
            op: Operation::Sub,
            left,
            right,
        } => Ok(eval(*left)? - eval(*right)?),
        Expression::Op {
            op: Operation::Mul,
            left,
            right,
        } => Ok(eval(*left)? * eval(*right)?),
        Expression::Op {
            op: Operation::Div,
            left,
            right,
        } => {
            let right_calc = eval(*right)?;
            if (right_calc == 0) {
                Err(String::from("division by zero"))
            } else {
                Ok(eval(*left)? / right_calc)
            }
        }
        Expression::Value(value) => Ok(value)
    }
}
