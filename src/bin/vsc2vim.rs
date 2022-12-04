// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use clap::Parser;
use std::process;
use vsc2vim::{convert, Args};

fn main() {
    let args = Args::parse();

    // Format error ourselves: https://github.com/rust-lang/project-error-handling/issues/39
    if let Err(err) = convert(&args) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
