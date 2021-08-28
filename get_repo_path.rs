// Deps:
// camino = "1.0"

use camino::Utf8PathBuf;

/// Get the absolute path of the repo. Assumes that this executable is
/// located at <repo>/target/<buildmode>/<exename>.
fn get_repo_path() -> Option<Utf8PathBuf> {
    let exe = env::current_exe().ok()?;
    let repo = exe.parent()?.parent()?;
    Utf8PathBuf::from_path_buf(repo.into()).ok()
}
