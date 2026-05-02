use rustyline::validate::{ValidationContext, ValidationResult, Validator};

#[derive(Debug, PartialEq)]
enum QuoteState {
    Normal,
    Single,
    Double,
    EscapedInNormal,
    EscapedInDouble,
}

#[derive(Default)]
pub struct ShellValidator;

impl ShellValidator {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Validator for ShellValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        let input = ctx.input();

        match check_input(input) {
            InputStatus::Complete     => Ok(ValidationResult::Valid(None)),
            InputStatus::Incomplete   => Ok(ValidationResult::Incomplete),
            InputStatus::Invalid(msg) => Ok(ValidationResult::Invalid(
                Some(format!("\n  error: {}", msg))
            )),
        }
    }
}

