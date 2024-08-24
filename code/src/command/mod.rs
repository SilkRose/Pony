#![deny(missing_docs)]
#![doc = include_str!("./readme.md")]

use std::process::{Command, ExitStatus, Output};

type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

/// Executes a Linux/macOS or Windows command with a Result of ExitStatus.
pub fn execute_command(cmd: &str) -> Result<ExitStatus> {
	#[cfg(not(target_os = "windows"))]
	return Ok(Command::new("sh").arg("-c").arg(cmd).status()?);
	#[cfg(target_os = "windows")]
	return Ok(Command::new("cmd").args(["/C", cmd]).status()?);
}

/// Executes a Linux/macOS or Windows command with a Result of the output.
pub fn execute_command_with_return(cmd: &str) -> Result<Output> {
	#[cfg(not(target_os = "windows"))]
	return Ok(Command::new("sh").arg("-c").arg(cmd).output()?);
	#[cfg(target_os = "windows")]
	return Ok(Command::new("cmd").args(["/C", cmd]).output()?);
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::error::Error;
	#[test]
	fn command_status() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_command("exit 0")?;
		assert!(status.success());
		Ok(())
	}
	#[test]
	fn command_status_failure() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_command("exit 1")?;
		assert!(!status.success());
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
		let status = execute_command("exit 0")?;
		assert!(status.success());
		Ok(())
	}
	#[test]
	#[cfg(not(target_os = "windows"))]
	fn unix_status_failure() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_command("exit 1")?;
		assert!(!status.success());
		Ok(())
	}
	#[test]
	#[cfg(target_os = "windows")]
	fn windows_status() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_command("exit 0")?;
		assert!(status.success());
		Ok(())
	}
	#[test]
	#[cfg(target_os = "windows")]
	fn windows_status_failure() -> std::result::Result<(), Box<dyn Error>> {
		let status = execute_command("exit 1")?;
		assert!(!status.success());
		Ok(())
	}
	#[test]
	#[cfg(not(target_os = "windows"))]
	fn unix_output() -> std::result::Result<(), Box<dyn Error>> {
		let answer = "Pinkie Pie is best pony!";
		let output = execute_command_with_return(&format!("echo '{answer}'"))?;
		assert_eq!(String::from_utf8(output.stdout)?, format!("{answer}\n"));
		Ok(())
	}
	#[test]
	#[cfg(target_os = "windows")]
	fn windows_output() -> std::result::Result<(), Box<dyn Error>> {
		let answer = "Pinkie Pie is best pony!";
		let output = execute_command_with_return(&format!("echo '{answer}'"))?;
		assert_eq!(String::from_utf8(output.stdout)?, format!("{answer}\n"));
		Ok(())
	}
}
