use std::{collections::HashMap, env};
pub struct LongOp {
    pub short_op: String,
    pub long_op: String,
    pub usage: String,
    pub has_arg: bool,
}

impl LongOp {
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

    pub fn has_arg(mut self, has_arg: bool) -> Self {
        self.has_arg = has_arg;
        self
    }
}

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

pub fn get_op_map(ops: &[LongOp]) -> Result<HashMap<String, String>, String> {
    //make map to store ops
    let mut op_map: HashMap<String, String> = HashMap::new();
    let mut args = env::args().skip(1);
    //get all enviroment args skipping first one(is the program name)
    while let Some(arg) = args.next() {
        //match to current option
        let curr_op: Option<&LongOp> = if arg.starts_with("--") {
            ops.iter().find(|&op| op.long_op == arg[2..])
        } else if arg.starts_with("-") {
            ops.iter().find(|&op| op.short_op == arg[1..])
        } else {
            return Err(format!("Expected but - or -- but got {}", arg));
        };

        //map option to argument
        match curr_op {
            Some(op) => {
                if op.has_arg {
                    let arg_val: String = args
                        .next()
                        .ok_or(format!("Expected argument but none found"))?;
                    op_map.insert(op.long_op.clone(), arg_val);
                } else {
                    op_map.insert(op.long_op.clone(), "".into());
                }
            }
            None => {
                return Err(format!("Unknown Option {}", arg));
            }
        }
    }

    Ok(op_map)
}
