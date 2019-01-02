#[macro_use]
extern crate failure;

use failure::Error;
use std::process::Command;
use std::process::Output;

/// Represents a failed command (non-zero satus)
#[derive(Fail, Debug)]
#[fail(display = "Command failed: {}, output: {:?}", command, output)]
pub struct CommandFail {
	command: String,
	output: Output,
}

impl CommandFail {
	/// The casted command that has failed
	pub fn command(&self) -> &str {
		&self.command
	}

	/// The command output
	pub fn output(&self) -> &Output {
		&self.output
	}
}

type Result<T> = std::result::Result<T, Error>;

/// Runs the provided shell command, can return a CommandFail or other lower level errors (using failure)
pub fn run_shell_command(cmd: &str) -> Result<String> {
	let output = if cfg!(target_os = "windows") {
		// untested
		Command::new("cmd").arg("/C").arg(cmd).output()?
	} else {
		Command::new("sh").arg("-c").arg(cmd).output()?
	};
	if output.status.success() {
		Ok(String::from_utf8(output.stdout)?)
	} else {
		Err(CommandFail {
			command: cmd.into(),
			output,
		}
		.into())
	}
}
