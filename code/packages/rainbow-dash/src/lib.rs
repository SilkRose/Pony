#![deny(missing_docs)]
#![doc = include_str!("../readme.md")]

use std::io::Result;
use std::process::{Command, ExitStatus, Output};

/// Executes a command with a Result of ExitStatus.
/// Picks the correct function for Unix or Windows.
pub fn execute_command(cmd: &str) -> Result<ExitStatus> {
	#[cfg(not(target_os = "windows"))]
	return execute_unix_command(cmd);
	#[cfg(target_os = "windows")]
	return execute_windows_command(cmd);
}

/// Executes a command with a Result of ExitStatus.
/// Picks the correct function for Unix or Windows.
pub fn execute_command_with_return(cmd: &str) -> Result<Output> {
	#[cfg(not(target_os = "windows"))]
	return execute_unix_command_with_return(cmd);
	#[cfg(target_os = "windows")]
	return execute_windows_command_with_return(cmd);
}

/// Executes a Windows command with a Result of ExitStatus.
pub fn execute_windows_command(cmd: &str) -> Result<ExitStatus> {
	Command::new("cmd").args(["/C", cmd]).status()
}

/// Executes a Unix command with a Result of ExitStatus.
pub fn execute_unix_command(cmd: &str) -> Result<ExitStatus> {
	Command::new("sh").arg("-c").arg(cmd).status()
}

/// Executes a Windows command with a Result of the output.
pub fn execute_windows_command_with_return(cmd: &str) -> Result<Output> {
	Command::new("cmd").args(["/C", cmd]).output()
}

/// Executes a Unix command with a Result of the output.
pub fn execute_unix_command_with_return(cmd: &str) -> Result<Output> {
	Command::new("sh").arg("-c").arg(cmd).output()
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::error::Error;
	#[test]
	fn command_status() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_command("exit 0")?;
		assert_eq!(status.success(), true);
		Ok(())
	}
	#[test]
	fn command_status_failure() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_command("exit 1")?;
		assert_eq!(status.success(), false);
		Ok(())
	}
	#[test]
	fn command_output() -> std::result::Result<(), Box<dyn Error>> {
		let answer = "Pinkie Pie is best pony!";
		let output = execute_command_with_return(&format!("echo '{answer}'"))?;
		assert_eq!(String::from_utf8(output.stdout)?, format!("{answer}\n"));
		Ok(())
	}
	#[test]
	#[cfg(not(target_os = "windows"))]
	fn unix_status() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_unix_command("exit 0")?;
		assert_eq!(status.success(), true);
		Ok(())
	}
	#[test]
	#[cfg(not(target_os = "windows"))]
	fn unix_status_failure() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_unix_command("exit 1")?;
		assert_eq!(status.success(), false);
		Ok(())
	}
	#[test]
	#[cfg(target_os = "windows")]
	fn windows_status() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_windows_command("exit 0")?;
		assert_eq!(status.success(), true);
		Ok(())
	}
	#[test]
	#[cfg(target_os = "windows")]
	fn windows_status_failure() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_windows_command("exit 1")?;
		assert_eq!(status.success(), false);
		Ok(())
	}
	#[test]
	#[cfg(not(target_os = "windows"))]
	fn unix_output() -> std::result::Result<(), Box<dyn Error>> {
		let answer = "Pinkie Pie is best pony!";
		let output = execute_unix_command_with_return(&format!("echo '{answer}'"))?;
		assert_eq!(String::from_utf8(output.stdout)?, format!("{answer}\n"));
		Ok(())
	}
	#[test]
	#[cfg(target_os = "windows")]
	fn windows_output() -> std::result::Result<(), Box<dyn Error>> {
		let answer = "Pinkie Pie is best pony!";
		let output = execute_windows_command_with_return(&format!("echo '{answer}'"))?;
		assert_eq!(String::from_utf8(output.stdout)?, format!("{answer}\n"));
		Ok(())
	}
}
