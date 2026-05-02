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

#[derive(Debug, PartialEq)]
enum InputStatus {
    Complete,
    Incomplete,
    Invalid(String),
}

fn check_input(input: &str) -> InputStatus {
    let mut state      = QuoteState::Normal;
    let mut depth_paren: i32 = 0;
    let mut depth_brace: i32 = 0;

    for ch in input.chars() {
        match (&state, ch) {

            (QuoteState::EscapedInNormal, _) => {
                state = QuoteState::Normal;
            }

            (QuoteState::EscapedInDouble, _) => {
                state = QuoteState::Double;
            }

            (QuoteState::Normal, '\\') => {
                state = QuoteState::EscapedInNormal;
            }

            (QuoteState::Normal, '\'') => {
                state = QuoteState::Single;
            }

            (QuoteState::Normal, '"') => {
                state = QuoteState::Double;
            }

            (QuoteState::Single, '\'') => {
                state = QuoteState::Normal;
            }

            (QuoteState::Double, '\\') => {
                state = QuoteState::EscapedInDouble;
            }

            (QuoteState::Double, '"') => {
                state = QuoteState::Normal;
            }

            (QuoteState::Normal, '(') => depth_paren += 1,
            (QuoteState::Normal, ')') => {
                depth_paren -= 1;
                if depth_paren < 0 {
                    return InputStatus::Invalid("unexpected `)`".to_owned());
                }
            }

            (QuoteState::Normal, '{') => depth_brace += 1,
            (QuoteState::Normal, '}') => {
                depth_brace -= 1;
                if depth_brace < 0 {
                    return InputStatus::Invalid("unexpected `}`".to_owned());
                }
            }

            _ => {}
        }
    }

    match state {
        QuoteState::Single
        | QuoteState::Double
        | QuoteState::EscapedInNormal
        | QuoteState::EscapedInDouble => return InputStatus::Incomplete,
        QuoteState::Normal => {}
    }

    if depth_paren > 0 { return InputStatus::Incomplete; }
    if depth_brace > 0 { return InputStatus::Incomplete; }

    InputStatus::Complete
}