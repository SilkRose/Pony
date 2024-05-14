# Rainbow Dash

Command execution library for Linux/macOS and Windows.

Has functions with status and output variants.

## Example usage:

Running a command that is the same on Linux/macOS and Windows:
```rust
let status = execute_command("echo 'Pinkie Pie is best pony!'")?;
println!("{}", status.success()); // true or false
```

Running a command with output that is the same on Linux/macOS and Windows:
```rust
let output = execute_command_with_return("echo 'Pinkie Pie is best pony!'")?;
println!("{}", String::from_utf8(output.stdout)?); // Pinkie Pie is best pony!
```

If your commands are different on Linux/macOS and Windows, you can use the target-specific functions:
```rust
#[cfg(not(target_os = "windows"))]
let status = execute_unix_command("ls -la");
#[cfg(target_os = "windows")]
let status = execute_windows_command("dir /a /w");
println!("{}", status.success()); // true or false
```
