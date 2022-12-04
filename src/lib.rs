// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

mod cmd;
mod error;

pub use cmd::*;
pub use error::*;

pub fn convert(args: &Args) -> Result<()> {
    if !args.path.is_file() {
        return Err(Error::FileNotFound {
            path: args.path.to_owned(),
        });
    }

    println!("{:?}", args);
    Ok(())
}
