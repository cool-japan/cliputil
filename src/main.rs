use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use tempfile::Builder;
use tempfile::NamedTempFile;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cliputil <command> [options]");
        println!("Commands: execute, format, replace, transform");
        return Ok(());
    }

    let command = &args[1];
    match command.as_str() {
        "execute" => execute_code(&args[2..])?,
        "format" => format_code(&args[2..])?,
        "replace" => replace_text(&args[2..])?,
        "transform" => transform_text(&args[2..])?,
        _ => println!("Unknown command: {}", command),
    }

    Ok(())
}

fn execute_code(args: &[String]) -> io::Result<()> {
    let mut language = String::from("python");
    let mut verbose = false;
    let mut output_to_clipboard = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-l" | "--language" => {
                i += 1;
                if i < args.len() {
                    language = args[i].clone();
                }
            }
            "-v" | "--verbose" => verbose = true,
            "-o" | "--output-clipboard" => output_to_clipboard = true,
            _ => break,
        }
        i += 1;
    }

    let code = get_clipboard_content()?;

    if verbose {
        println!("Executing {} code:\n{}", language, code);
    }

    let temp_dir = Builder::new().prefix("cliputil").tempdir()?;
    let file_extension = match language.as_str() {
        "python" => "py",
        "javascript" => "js",
        "rust" => "rs",
        "cpp" => "cpp",
        "java" => "java",
        "php" => "php",
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported language",
            ))
        }
    };
    let temp_file_path = temp_dir
        .path()
        .join(format!("temp_code.{}", file_extension));
    fs::write(&temp_file_path, &code)?;

    let output = match language.as_str() {
        "python" => execute_command("python", &[temp_file_path.to_str().unwrap()], &args[i..])?,
        "javascript" => execute_command("node", &[temp_file_path.to_str().unwrap()], &args[i..])?,
        "rust" => {
            let executable_path = temp_dir.path().join("temp_executable");
            compile_and_run("rustc", &temp_file_path, &executable_path, &args[i..])?
        }
        "cpp" => {
            let executable_path = temp_dir.path().join("temp_executable");
            compile_and_run("g++", &temp_file_path, &executable_path, &args[i..])?
        }
        "java" => {
            let class_name = extract_java_class_name(&code).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Could not find Java class name",
                )
            })?;
            let java_file_path = temp_dir.path().join(format!("{}.java", class_name));
            fs::write(&java_file_path, &code)?;
            let compile_output = Command::new("javac").arg(&java_file_path).output()?;
            if !compile_output.status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "Java compilation failed: {}",
                        String::from_utf8_lossy(&compile_output.stderr)
                    ),
                ));
            }
            execute_command(
                "java",
                &["-cp", temp_dir.path().to_str().unwrap(), &class_name],
                &args[i..],
            )?
        }
        "php" => execute_command("php", &[temp_file_path.to_str().unwrap()], &args[i..])?,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported language",
            ))
        }
    };

    if output_to_clipboard {
        set_clipboard_content(&String::from_utf8_lossy(&output.stdout))?;
        println!("Execution result copied to clipboard.");
    } else {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
    }

    Ok(())
}

fn execute_command(
    command: &str,
    args: &[&str],
    extra_args: &[String],
) -> io::Result<std::process::Output> {
    Command::new(command).args(args).args(extra_args).output()
}

fn compile_and_run(
    compiler: &str,
    source_path: &Path,
    executable_path: &Path,
    args: &[String],
) -> io::Result<std::process::Output> {
    let compile_output = Command::new(compiler)
        .arg(source_path)
        .arg("-o")
        .arg(executable_path)
        .output()?;

    if !compile_output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Compilation failed: {}",
                String::from_utf8_lossy(&compile_output.stderr)
            ),
        ));
    }

    Command::new(executable_path).args(args).output()
}

fn format_code(args: &[String]) -> io::Result<()> {
    let mut language = String::from("python");
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-l" | "--language" => {
                i += 1;
                if i < args.len() {
                    language = args[i].clone();
                }
            }
            _ => break,
        }
        i += 1;
    }

    let code = get_clipboard_content()?;

    let formatted_code = match language.as_str() {
        "python" => format_with_command("black", &["--quiet", "-"], &code)?,
        "javascript" => format_with_command("prettier", &["--stdin-filepath", "code.js"], &code)?,
        "rust" => format_with_command("rustfmt", &[], &code)?,
        "cpp" | "c++" => format_with_command(
            "clang-format",
            &["-style=file", "-assume-filename=.cpp"],
            &code,
        )?,
        "java" => format_with_command(
            "java",
            &["-jar", "/usr/local/bin/google-java-format.jar", "-"],
            &code,
        )?,
        "php" => {
            let temp_file = NamedTempFile::new()?;
            fs::write(temp_file.path(), code)?;
            let output = Command::new("php-cs-fixer")
                .args(&[
                    "fix",
                    "--rules=@PSR2",
                    "--diff",
                    temp_file.path().to_str().unwrap(),
                ])
                .output()?;
            if !output.status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "php-cs-fixer failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ),
                ));
            }
            fs::read_to_string(temp_file.path())?
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unsupported language",
            ))
        }
    };

    set_clipboard_content(&formatted_code)?;
    println!("Formatted code copied to clipboard.");
    Ok(())
}

fn format_with_command(command: &str, args: &[&str], input: &str) -> io::Result<String> {
    let mut child = Command::new(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            if e.kind() == io::ErrorKind::NotFound {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "Command '{}' not found. Make sure it's installed and in your PATH.",
                        command
                    ),
                )
            } else {
                e
            }
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())?;
    } else {
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to open stdin"));
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "{} failed: {}\nStdout: {}",
                command,
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            ),
        ));
    }

    if output.stdout.is_empty() {
        Ok(input.to_string())
    } else {
        String::from_utf8(output.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

fn replace_text(args: &[String]) -> io::Result<()> {
    if args.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not enough arguments",
        ));
    }

    let text = get_clipboard_content()?;
    let replaced_text = text.replace(&args[0], &args[1]);
    set_clipboard_content(&replaced_text)?;
    println!("Text replaced and copied to clipboard.");

    Ok(())
}

fn extract_java_class_name(code: &str) -> Option<String> {
    let re = regex::Regex::new(r"public\s+class\s+(\w+)").ok()?;
    re.captures(code)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

fn transform_text(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No transformation specified",
        ));
    }

    let text = get_clipboard_content()?;
    let transformed_text = match args[0].as_str() {
        "upper" => text.to_uppercase(),
        "lower" => text.to_lowercase(),
        "title" => text
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>()
                            + &chars.collect::<String>().to_lowercase()
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(" "),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unknown transformation",
            ))
        }
    };

    set_clipboard_content(&transformed_text)?;
    println!("Text transformed and copied to clipboard.");

    Ok(())
}

fn get_clipboard_content() -> io::Result<String> {
    let output = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .arg("-o")
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn set_clipboard_content(content: &str) -> io::Result<()> {
    let mut child = Command::new("xclip")
        .arg("-selection")
        .arg("clipboard")
        .stdin(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(content.as_bytes())?;
    child.wait()?;

    Ok(())
}
