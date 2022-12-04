// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the Visual Studio Code theme to convert.
    #[arg(required = true)]
    pub(crate) path: PathBuf,

    /// Displays the palette of colors.
    #[arg(long)]
    pub(crate) palette: bool,
}
