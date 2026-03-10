// src/error.rs

#[derive(Debug)]
pub enum RshError {
    CommandNotFound(String),
    SpawnFailed(String, std::io::Error),
    WaitFailed(String, std::io::Error),
    ReadFailed(std::io::Error),
    FlushFailed(std::io::Error),
    CdFailed(String, std::io::Error),
    CwdFailed(std::io::Error),
}

impl std::fmt::Display for RshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandNotFound(cmd) => write!(f, "rsh: command not found: {cmd}"),
            Self::SpawnFailed(cmd, e)  => write!(f, "rsh: failed to run '{cmd}': {e}"),
            Self::WaitFailed(cmd, e)   => write!(f, "rsh: failed to wait on '{cmd}': {e}"),
            Self::ReadFailed(e)        => write!(f, "rsh: failed to read input: {e}"),
            Self::FlushFailed(e)       => write!(f, "rsh: failed to flush stdout: {e}"),
            Self::CdFailed(path, e)    => write!(f, "cd: {path}: {e}"),
            Self::CwdFailed(e)         => write!(f, "rsh: failed to get cwd: {e}"),
        }
    }
}
