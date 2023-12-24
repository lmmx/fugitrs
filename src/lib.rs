use pyo3::prelude::*;
use git2::{Repository, Diff, DiffOptions};

#[pyfunction]
fn get_git_diff(repo_path: String, revision: String) -> PyResult<()> {
    // Open the repository
    let repo = Repository::open(repo_path).expect("Failed to open repository");

    // Determine the revisions
    let revspec = repo.revparse(&revision).expect("Failed to parse revision");

    let (from, to) = (revspec.from().map(|r| r.peel_to_tree().unwrap()),
                      revspec.to().map(|r| r.peel_to_tree().unwrap()));

    // Create diff
    let diff = repo.diff_tree_to_tree(from.as_ref(), to.as_ref(), None)
                   .expect("Failed to create diff");

    // Iterate over diff and print each line
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
        let line_content = std::str::from_utf8(line.content()).unwrap_or("");
        println!("{}", line_content);
        true
    }).expect("Failed to print diff");

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn git_diff_extension(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_git_diff, m)?)?;

    Ok(())
}
