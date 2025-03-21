mod error;
pub(crate) use error::Error;

fn load_dotenv() -> Result<(), Error> {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(dotenvy::Error::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(dotenvy::Error::Io(e)) => return Err(Error::DotenvyIo(e)),
        Err(dotenvy::Error::LineParse(line_content, line_number)) => {
            return Err(Error::DotenvyParse {
                line_content,
                line_number,
            });
        }
        Err(dotenvy::Error::EnvVar(_)) => {
            unreachable!("this error is never returned from dotenvy::dotenv")
        }
        Err(_) => todo!(),
    }

    Ok(())
}

pub fn run() -> Result<(), Error> {
    load_dotenv()?;

    Ok(())
}
