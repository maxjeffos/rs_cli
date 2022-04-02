#[derive(Debug)]
pub enum Error {
    Error1,
    Error2,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Error1 => write!(f, "Error 1"),
            Error::Error2 => write!(f, "Error 2"),
        }
    }
}

pub fn get_answer() -> i32 {
    42
}

pub fn print_stuff(mut writer: impl std::io::Write) -> std::io::Result<()> {
    writeln!(writer, "Hello, world!")?;
    Ok(())
}

pub fn might_return_error() -> Result<(), Error> {
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn get_answer_works() {
        assert_eq!(get_answer(), 42);
    }

    #[test]
    fn print_stuff_works() -> std::io::Result<()> {
        let mut writer = Vec::new(); // cool, a Vec can be a writer
        print_stuff(&mut writer)?;

        assert_eq!(writer, b"Hello, world!\n");
        // assert_eq!(String::from_utf8(writer).unwrap(), "Hello, world!\n"); // this also works
        
        Ok(())
    }
}
