use std::process::{Command, Output};

pub fn execute_windows_command(cmd: &str) {
	let output = Command::new("cmd")
		.args(["/C", cmd])
		.output()
		.unwrap_or_else(|_| panic!("failed to execute: {cmd}"));

	if !output.status.success() {
		panic!("failed to execute: {cmd}")
	}
}

pub fn execute_unix_command(cmd: &str) {
	let output = Command::new("sh")
		.arg("-c")
		.arg(cmd)
		.output()
		.unwrap_or_else(|_| panic!("failed to execute: {cmd}"));

	if !output.status.success() {
		panic!("failed to execute: {cmd}")
	}
}

pub fn execute_windows_command_with_return(cmd: &str) -> Output {
	let output = Command::new("cmd")
		.args(["/C", cmd])
		.output()
		.unwrap_or_else(|_| panic!("failed to execute: {cmd}"));

	if !output.status.success() {
		panic!("failed to execute: {cmd}")
	} else {
		output
	}
}

pub fn execute_unix_command_with_return(cmd: &str) -> Output {
	let output = Command::new("sh")
		.arg("-c")
		.arg(cmd)
		.output()
		.unwrap_or_else(|_| panic!("failed to execute: {cmd}"));

	if !output.status.success() {
		panic!("failed to execute: {cmd}")
	} else {
		output
	}
}

pub fn execute_windows_command_with_fail_msg(cmd: &str, msg: &str) {
	let output = Command::new("cmd")
		.args(["/C", cmd])
		.output()
		.unwrap_or_else(|_| panic!("{msg}"));

	if !output.status.success() {
		panic!("{msg}")
	}
}

pub fn execute_unix_command_with_fail_msg(cmd: &str, msg: &str) {
	let output = Command::new("sh")
		.arg("-c")
		.arg(cmd)
		.output()
		.unwrap_or_else(|_| panic!("{msg}"));

	if !output.status.success() {
		panic!("{msg}")
	}
}
