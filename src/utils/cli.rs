// Copyright (c) 2026 Seth Holtzman
// SPDX-License-Identifier: MIT
// Author: Seth Holtzman
// See LICENSE file in the project root for full license text.

use crate::utils::errors::UtilError;
use crate::utils::errors::UtilError::CLIError;
use std::{collections::HashMap, env};

/// Represents a command line option with both a short and long form.
///
/// # Examples
/// ```
/// use warrior_util::utils::cli::{LongOp};
///
/// let op = LongOp::new("p", "port", "Port to use.");
/// let op_no_arg = LongOp::new("h", "help", "Display help.").has_arg(false);
/// ```
#[derive(Hash, Eq, PartialEq)]
pub struct LongOp {
    pub short_op: String,
    pub long_op: String,
    pub usage: String,
    pub has_arg: bool,
}

impl LongOp {
    /// Creates a new `LongOp` with the given short and long forms and usage description.
    /// By default `has_arg` is `true`.
    ///
    /// # Arguments
    /// * `short_op` - The short form of the option e.g. `p` for `-p`
    /// * `long_op` - The long form of the option e.g. `port` for `--port`
    /// * `usage` - Description of what the option does
    pub fn new(
        short_op: impl Into<String>,
        long_op: impl Into<String>,
        usage: impl Into<String>,
    ) -> Self {
        LongOp {
            short_op: short_op.into(),
            long_op: long_op.into(),
            usage: usage.into(),
            has_arg: true,
        }
    }
    /// Sets whether this long option expects a value after it.
    ///
    /// # Examples
    /// ```
    /// use warrior_util::utils::cli::{LongOp};
    ///
    /// let op = LongOp::new("h", "help", "Display help.").has_arg(false);
    /// ```
    pub fn has_arg(mut self, has_arg: bool) -> Self {
        self.has_arg = has_arg;
        self
    }
}

/// Returns a formatted string of all options for display in help text.
///
/// # Arguments
/// * `ops` - Slice of `LongOp` to display
///
/// # Examples
/// ```
/// use warrior_util::utils::cli::{LongOp, options_string};
///
/// let ops = vec![LongOp::new("p", "port", "Port to use.")];
/// println!("{}", options_string(&ops));
/// // Options:
/// //   -p, --port      Port to use.
/// ```
pub fn options_string(ops: &[LongOp]) -> String {
    let mut builder: String = String::from("Options: \n");
    for op in ops {
        builder.push_str(&format!(
            "  -{}, --{} \t\t{}\n",
            op.short_op, op.long_op, op.usage
        ))
    }
    builder
}

/// Parses command line arguments and maps them to their corresponding `LongOp`.
///
/// Returns a `HashMap` mapping each matched `LongOp` to its value, or an empty
/// string if the option takes no argument.
///
/// # Arguments
/// * `ops` - Slice of valid `LongOp` to match against
///
/// # Errors
/// * If an argument doesn't start with `-` or `--`
/// * If an unknown option is provided
/// * If an option that requires a value is not followed by one
pub fn get_op_map(ops: &[LongOp]) -> Result<HashMap<&LongOp, String>, UtilError> {
    //make map to store ops
    let mut op_map: HashMap<&LongOp, String> = HashMap::new();
    let mut args = env::args().skip(1);
    //get all enviroment args skipping first one(is the program name)
    while let Some(arg) = args.next() {
        //match to current option
        let curr_op: Option<&LongOp> = if arg.starts_with("--") {
            ops.iter().find(|&op| op.long_op == arg[2..])
        } else if arg.starts_with("-") {
            ops.iter().find(|&op| op.short_op == arg[1..])
        } else {
            return Err(CLIError(format!("Expected but - or -- but got {}", arg)));
        };

        //map option to argument
        match curr_op {
            Some(op) => {
                if op.has_arg {
                    let arg_val: String = args
                        .next()
                        .ok_or(CLIError(format!("Expected argument but none found")))?;
                    op_map.insert(op, arg_val);
                } else {
                    op_map.insert(op, "".into());
                }
            }
            None => {
                return Err(CLIError(format!("Unknown Option {}", arg)));
            }
        }
    }

    Ok(op_map)
}
