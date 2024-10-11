pub struct Calc;

impl Calc {
    pub fn calculate(input: &str) -> f64 {
        let mut num = String::new();
        let mut stack = Vec::new();
        let mut op_stack = Vec::new();
        let mut input = input.chars().peekable();

        fn apply_op(op: char, b: f64, a: f64) -> f64 {
            match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                '^' => a.powf(b),
                _ => unreachable!(),
            }
        }

        fn precedence(op: char) -> i32 {
            match op {
                '+' | '-' => 1,
                '*' | '/' => 2,
                '^' => 3,
                _ => 0,
            }
        }

        while let Some(&c) = input.peek() {
            match c {
                '0'..='9' | '.' => {
                    num.push(c);
                    input.next();
                }
                '+' | '-' | '*' | '/' | '^' => {
                    if !num.is_empty() {
                        stack.push(num.parse::<f64>().unwrap());
                        num.clear();
                    }
                    while let Some(&top_op) = op_stack.last() {
                        if precedence(top_op) >= precedence(c) {
                            let b = stack.pop().unwrap();
                            let a = stack.pop().unwrap();
                            stack.push(apply_op(op_stack.pop().unwrap(), b, a));
                        } else {
                            break;
                        }
                    }
                    op_stack.push(c);
                    input.next();
                }
                '(' => {
                    op_stack.push(c);
                    input.next();
                }
                ')' => {
                    if !num.is_empty() {
                        stack.push(num.parse::<f64>().unwrap());
                        num.clear();
                    }
                    while let Some(&top_op) = op_stack.last() {
                        if top_op == '(' {
                            op_stack.pop();
                            break;
                        } else {
                            let b = stack.pop().unwrap();
                            let a = stack.pop().unwrap();
                            stack.push(apply_op(op_stack.pop().unwrap(), b, a));
                        }
                    }
                    input.next();
                }
                _ => {
                    input.next();
                }
            }
        }

        if !num.is_empty() {
            stack.push(num.parse::<f64>().unwrap());
        }

        while let Some(op) = op_stack.pop() {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(apply_op(op, b, a));
        }

        stack.pop().unwrap()
    }
}
