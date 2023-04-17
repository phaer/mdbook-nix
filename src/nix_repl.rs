use std::process::Command;

use rexpect::spawn_with_options;
use rexpect::session::{PtyReplSession, Options};
use anyhow::Result;


pub struct Repl {
    session: PtyReplSession
}


impl Repl {
    pub fn new() -> Result<Self> {
        let mut nix_repl = Command::new("/run/current-system/sw/bin/nix"); // TODO
        nix_repl.args(["repl"]);
        let mut session = PtyReplSession {
            echo_on: true,
            prompt: "nix-repl> ".to_string(),
            pty_session: spawn_with_options(nix_repl, Options {
                timeout_ms: Some(2000),
                strip_ansi_escape_codes: true
            })?,
            quit_command: Some(":q".to_string())
        };
        session.wait_for_prompt()?;
        Ok(Repl {session})
    }

    pub fn eval(&mut self, input: &str) -> Result<String> {
        for line in input.lines() {
            if line.starts_with("#") {
                // we need to filter comments, because nix repl
                // doesn't really handle them
                continue
            }
            self.session.send_line(line)?;
        }
        let (_, result) = self.session.exp_regex(r"\n.*")?;
        self.session.wait_for_prompt()?;
        Ok(result.trim().to_string())
    }
}
