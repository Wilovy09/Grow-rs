use srtemplate::function::{self, FuncResult};
use srtemplate::prelude::validations;

use super::fake_generated;

pub fn fake(args: &[String]) -> FuncResult {
    validations::args_min_len(args, 1)?;
    validations::args_max_len(args, 1)?;

    let kind = &args[0];
    let Ok(kind) = kind.parse::<u16>() else {
        return Err(function::Error::InvalidType(kind.to_owned()));
    };

    let Some(result) = fake_generated::execute_faker(kind) else {
        return Err(function::Error::RuntimeError(format!(
            "Fake kind is not valid: {kind}"
        )));
    };

    Ok(result)
}
