use std::fs::File;
use std::process::Command;
use std::process::Stdio;

fn main() -> std::io::Result<()> {
    let mut stdout_f = File::create("stdout.log")?;
    let mut stderr_f = File::create("stderr.log")?;
    let mut child = Command::new("./foo.py")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    if let Some(mut out) = child.stdout.take() {
        std::thread::spawn(move || {
            std::io::copy(&mut out, &mut stdout_f).expect("Failed to copy stdout");
        });
    }
    if let Some(mut err) = child.stderr.take() {
        std::io::copy(&mut err, &mut stderr_f).expect("Failed to copy stderr");
    }

    assert!(child.wait().expect("Joined process succefully").success());
    Ok(())
}
