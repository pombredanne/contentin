use std;
use std::error;
use std::io;

use ext4;
use zip;

error_chain! {
    links {
        Ext4(::ext4::Error, ::ext4::ErrorKind);
    }

    foreign_links {
        Io(io::Error);
        Zip(zip::result::ZipError);
    }

    errors {
        Rewind {
            description("bailin' out")
            display("rewind")
        }
        RecoverableFailure(msg: String) {
            description("a major problem, but we believe we can recover")
            display("invalid format: {}", msg)
        }
        UnsupportedFeature(msg: String) {
            description("format is (probably) legal, but we refuse to support its feature")
            display("unsupported feature: {}", msg)
        }
        Tar(msg: String) {
            description("tar-rs returned Other")
            display("tar failure message: {}", msg)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FormatErrorType {
    Rewind,
    Other,
}

pub fn classify_format_error_result<T>(res: &Result<T>) -> Option<FormatErrorType> {
    if res.is_ok() {
        return None;
    }

    let error = res.as_ref().err().unwrap();

    let broken_ref = error.iter().last().unwrap();

    if let Some(e) = unsafe_staticify(broken_ref).downcast_ref::<Error>() {
        is_format_error(e)
    } else if unsafe_staticify(broken_ref).is::<zip::result::ZipError>() {
        // Most zip errors should be wrapped in an errors::Error,
        // but https://github.com/brson/error-chain/issues/159

        // This is just a copy-paste of is_format_error's Zip(_) => Other
        Some(FormatErrorType::Other)
    } else if unsafe_staticify(broken_ref).is::<ext4::Error>() {
        // see zip comment above
        Some(FormatErrorType::Other)
    } else if let Some(e) = unsafe_staticify(broken_ref).downcast_ref::<io::Error>() {
        is_io_format_error(e).unwrap_or(None)
    } else {
        //            self.log(1, || format!("unexpectedly failed to match an error type: {:?}", broken_ref))?;
        None
    }
}

fn is_format_error(e: &Error) -> Option<FormatErrorType> {
    match *e.kind() {
        ErrorKind::Rewind => {
            return Some(FormatErrorType::Rewind);
        }
        ErrorKind::Tar(_) | ErrorKind::RecoverableFailure(_) | ErrorKind::UnsupportedFeature(_) => {
            return Some(FormatErrorType::Other);
        }
        ErrorKind::Io(ref e) => if let Some(result) = is_io_format_error(e) {
            return result;
        },

        ErrorKind::Ext4(_) => {
            return Some(FormatErrorType::Other);
        }

        ErrorKind::Zip(_) => {
            return Some(FormatErrorType::Other);
        }

        ErrorKind::Msg(_) => {
            return None;
        }
    }

    None
}

fn is_io_format_error(e: &io::Error) -> Option<Option<FormatErrorType>> {
    // if there's an actual error code (regardless of what it is),
    // it's probably not from a library
    if e.raw_os_error().is_some() {
        return Some(None);
    }

    match e.kind() {
        io::ErrorKind::InvalidData
        | io::ErrorKind::InvalidInput
        | io::ErrorKind::Other
        | io::ErrorKind::UnexpectedEof => return Some(Some(FormatErrorType::Other)),
        io::ErrorKind::BrokenPipe | io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied => {
            return Some(None)
        }
        _ => {}
    }

    None
}

/// UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE
/// UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE
/// UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE
/// UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE
/// UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE UNSAFE
///
/// TODO: working around https://github.com/rust-lang/rust/issues/35943
/// TODO: i.e. error.cause() is totally useless
///
/// This allows methods from the `Any` trait to be executed, e.g.
/// `is::<>` `and downcast_ref::<>`. I recommend you run them immediately;
/// i.e. don't even put the result of the method into a local.
fn unsafe_staticify(err: &error::Error) -> &'static error::Error {
    unsafe { std::mem::transmute(err) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::error::Error as Foo;
    use zip;

    fn simulate_failure(input: bool) -> zip::result::ZipResult<bool> {
        if !input {
            Ok(input)
        } else {
            Err(zip::result::ZipError::FileNotFound)
        }
    }

    fn simulate_second_failure(input: bool) -> zip::result::ZipResult<bool> {
        if !input {
            Ok(input)
        } else {
            Err(zip::result::ZipError::Io(io::ErrorKind::BrokenPipe.into()))
        }
    }

    #[test]
    fn real_format_error() {
        let failure = simulate_failure(true).chain_err(|| "oops");
        assert!(classify_format_error_result(&failure).unwrap() == FormatErrorType::Other);
    }

    #[test]
    fn io_error_is_not_format() {
        let err: io::Error = io::ErrorKind::BrokenPipe.into();
        let res: Result<()> = Err(err).chain_err(|| "oops");
        assert!(classify_format_error_result(&res).is_none())
    }

    #[test]
    fn nested_zip_failure_is_not_format() {
        let failure = simulate_second_failure(true).chain_err(|| "oops");
        assert!(classify_format_error_result(&failure).is_none())
    }

    #[test]
    fn chain_syntax_irrelevant() {
        let literate = simulate_failure(true).chain_err(|| "whoopsie").unwrap_err();
        let explicit = Error::with_chain(simulate_failure(true).unwrap_err(), "whoopsie");

        match literate.kind() {
            &ErrorKind::Msg(_) => {}
            _ => panic!(),
        };
        match explicit.kind() {
            &ErrorKind::Msg(_) => {}
            _ => panic!(),
        }

        let lit_cause = explicit.cause().unwrap();
        assert!(unsafe_staticify(lit_cause).is::<zip::result::ZipError>());

        let exp_cause = explicit.cause().unwrap();
        assert!(unsafe_staticify(exp_cause).is::<zip::result::ZipError>());
    }
}
