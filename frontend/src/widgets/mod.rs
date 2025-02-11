use std::fmt::Display;

pub mod login_form;
pub mod list_view;
pub mod registration_form;
pub mod password_change_form;
pub mod user_search;
pub mod new_room_form;

#[derive(Clone)]
enum AccountErrorReason {
    NoPassword,
    InvalidLengthPassword,
    BadConfirmPassword,
    EmptyUsername,
    InvalidLengthUsername,
}

impl Display for AccountErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountErrorReason::NoPassword => write!(f, "No password"),
            AccountErrorReason::InvalidLengthPassword => write!(f, "Invalid password length"),
            AccountErrorReason::BadConfirmPassword => write!(f, "Passwords do not match"),
            AccountErrorReason::EmptyUsername => write!(f, "No username"),
            AccountErrorReason::InvalidLengthUsername => write!(f, "Invalid username length")
        }
    }
}

fn username_offline_check(username: &str) -> Result<(), AccountErrorReason> {
    use AccountErrorReason::*;
    match username {
        name if name.is_empty() => Err(EmptyUsername),
        name if name.len() < 4 || name.len() > 64 => Err(InvalidLengthUsername),
        _ => Ok(())
    }
}

fn password_offline_check(password: &str, password_confirm: &str) -> Result<(), AccountErrorReason> {
    use AccountErrorReason::*;
    match (password, password_confirm) {
        (p, pc) if p.is_empty() && pc.is_empty() => Err(NoPassword),
        (p, pc) if p.ne(pc) => Err(BadConfirmPassword),
        (p, _) if p.len() < 8 || p.len() > 64 => Err(InvalidLengthPassword),
        _ => Ok(())
    }
}