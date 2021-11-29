use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::process::Stdio;


fn main() -> std::io::Result<()> {

    let mut stdout_f = File::create("stdout.log")?;
    let mut stderr_f = File::create("stderr.log")?;
    let output = Command::new("./foo.py")
                         .stdout(Stdio::piped())
                         .stderr(Stdio::piped())
                         .output()
                         .expect("failed to execute process");
    
    println!("status: {}", output.status);

    stdout_f.write_all(&output.stdout).unwrap();
    stderr_f.write_all(&output.stderr).unwrap();
    
    assert!(output.status.success());
    Ok(())
}
