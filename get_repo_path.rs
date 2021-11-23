// Deps:
// anyhow = "1.0"
// camino = "1.0"
// fehler = "1.0"

use anyhow::{anyhow, Error};
use camino::Utf8PathBuf;
use fehler::throws;
use std::env;

/// Get the absolute path of the repo. Assumes that this executable is
/// located at <repo>/target/<buildmode>/<exename>.
#[throws]
fn get_repo_path() -> Utf8PathBuf {
    let exe = Utf8PathBuf::from_path_buf(env::current_exe()?)
        .map_err(|_| anyhow!("exe path is not utf-8"))?;
    exe.parent()
        .map(|path| path.parent())
        .flatten()
        .map(|path| path.parent())
        .flatten()
        .ok_or_else(|| anyhow!("not enough parents: {}", exe))?
        .into()
}

// Version without camino:

/// Get the absolute path of the repo. Assumes that this executable is
/// located at <repo>/target/<buildmode>/<exename>.
#[throws]
fn get_repo_path() -> PathBuf {
    let exe = env::current_exe()?;
    exe.parent()
        .map(|path| path.parent())
        .flatten()
        .map(|path| path.parent())
        .flatten()
        .ok_or_else(|| anyhow!("not enough parents: {}", exe.display()))?
        .into()
}

// Option version:

/// Get the absolute path of the repo. Assumes that this executable is
/// located at <repo>/target/<buildmode>/<exename>.
fn get_repo_path() -> Option<Utf8PathBuf> {
    let exe = env::current_exe().ok()?;
    let repo = exe.parent()?.parent()?.parent()?;
    Utf8PathBuf::from_path_buf(repo.into()).ok()
}
