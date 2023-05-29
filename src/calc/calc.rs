use super::error::CalcError;
use super::expression::Expression;

pub struct Calc {
    expression: Expression,
    result: Option<f64>,
}

impl Calc {
    /// Creates a new `Calc` with `expression` empty and `result` equal to 0.0
    pub fn new() -> Self {
        Self { expression: Expression::new(), result: None }
    }

    pub fn result(&self) -> Option<f64> {
        self.result.clone()
    }

    pub fn evaluate(&mut self, value: &str) -> Result<f64, CalcError> {
        self.expression.set(value);
        self.expression.dijkstrify();

        let outcome = self.expression.calc()?;
        self.result = Some(outcome);
        Ok(outcome)
    }
}