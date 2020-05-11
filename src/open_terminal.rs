use super::CONFIG;
use std::process::Command;

// {DIR} -> dir to open

pub fn open_terminal(path: String) {

    let mut args = CONFIG.args.clone();
    let mut args = args.iter_mut().map(|v| v.replace("{DIR}", path.as_str())).collect::<Vec<String>>();
    let args = args.iter_mut().map(|v| v.as_str()).collect::<Vec<&str>>();

    let cmd = Command::new(&CONFIG.terminal).args(&args).spawn();
    drop(cmd);

}
