#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for input in inputs {
        match input {
            CalculatorInput::Value(value) => stack.push(*value),
            CalculatorInput::Add => {
                if let Some(b) = stack.pop() {
                    if let Some(a) = stack.pop() {
                        stack.push(a + b);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            },
            CalculatorInput::Subtract => {
                if let Some(b) = stack.pop() {
                    if let Some(a) = stack.pop() {
                        stack.push(a - b);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            },
            CalculatorInput::Multiply => {
                if let Some(b) = stack.pop() {
                    if let Some(a) = stack.pop() {
                        stack.push(a * b);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            },
            CalculatorInput::Divide => {
                if let Some(b) = stack.pop() {
                    if let Some(a) = stack.pop() {
                        if b == 0 {
                            return None;
                        } else {
                            stack.push(a / b);
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            },
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

