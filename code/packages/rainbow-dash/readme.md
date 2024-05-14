# Rainbow Dash

Command execution library.

Has functions for Linux/macOS and Windows with status and output variants.

## Example usage:

Running a command:
```rust
#[cfg(not(target_os = "windows"))]
let status = execute_unix_command("echo 'Pinkie Pie is best pony!'")?;

#[cfg(target_os = "windows")]
let status = execute_windows_command("echo 'Pinkie Pie is best pony!'")?;

if !status.success() {
	/* handdle failure */			
}
```

