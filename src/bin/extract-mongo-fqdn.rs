use std::process::Command;

fn main() {
  // you need to have a working local docker instance of agent connect running
  Command::new("bash")
    .arg("shell/upload-mongo-extract-script.sh")
    .spawn()
    .expect("command failed to start");
}
