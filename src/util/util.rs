use crate::util::error::Error;
use std::path::PathBuf;

pub fn path_to_string(path: &PathBuf) -> Result<String, Error> {
    path.clone().into_os_string().into_string().map_err(|_| {
        Error::new(
            String::from("error on OsString to String conversion"),
            String::from("maybe the path is not correct"),
        )
    })
}
