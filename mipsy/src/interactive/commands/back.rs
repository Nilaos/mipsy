use crate::interactive::{error::CommandError, prompt};

use super::*;
use colored::*;
use util::expect_u32;

pub(crate) fn back_command() -> Command {
    command(
        "back",
        vec!["b"],
        vec![],
        vec!["times"],
        &format!("step backwards one (or {}) instruction", "[times]".magenta()),
        &format!(
            "Steps backwards one instruction, or {0} instructions if specified.\n\
             It will then print out which instruction will be executed next --\n\
         \x20 i.e. using `{1}` will immediately execute said printed instruction.\n\
             To step fowards (i.e. normal stepping), use `{1}`.",
            "[times]".magenta(),
            "step".bold(),
        ),
        |state, _label, args| {
            let times = match args.first() {
                Some(arg) => expect_u32(
                    "back",
                    &"[times]".bright_magenta().to_string(),
                    arg, 
                    Some(|neg| 
                        format!("try `{}{}`", "step ".bold(), (-1 * neg as i32).to_string().bold())
                    )
                ),
                None => Ok(1),
            }?;

            let mut backs = 0;
            for _ in 0..times {
                let runtime = state.runtime.as_mut().ok_or(CommandError::MustLoadFile)?;

                if runtime.back() {
                    backs += 1;
                    state.exited = false;
                } else {
                    if backs == 0 {
                        return Err(CommandError::CannotStepFurtherBack);
                    }
                }
            }

            let binary  = state.binary.as_ref().ok_or(CommandError::MustLoadFile)?;
            let runtime = state.runtime.as_ref().ok_or(CommandError::MustLoadFile)?;

            let pluralise = if backs != 1 { "s" } else { "" };

            let mut text = String::from(format!("stepped back {} instruction{}", backs.to_string().magenta(), pluralise));
            if backs < times {
                text.push_str(" (reached start of program)");
            }
            text.push_str(", next instruction will be:");

            prompt::success(text);
            if let Ok(inst) = runtime.next_inst() {
                util::print_inst(&state.iset, binary, inst, runtime.state().get_pc());
            }
            println!();

            Ok(())
        }
    )
}
