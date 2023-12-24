use pyo3::prelude::*;
use git2::{Repository, DiffFormat, DiffLineType};

#[pyfunction]
fn get_git_diff(repo_path: String) -> PyResult<()> {
    let repo = Repository::open(repo_path).expect("Failed to open repository");

    // Get the tree of the current HEAD commit
    let head_commit = repo.head()
                          .and_then(|head| head.resolve())
                          .and_then(|head| head.peel_to_commit())
                          .expect("Failed to find head commit");
    let head_tree = head_commit.tree().expect("Failed to get head tree");

    // Perform a diff between the HEAD tree and the working directory
    let diff = repo.diff_tree_to_workdir_with_index(Some(&head_tree), None)
                   .expect("Failed to create diff");

    // Print the diff with line prefixes
    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        let line_content = std::str::from_utf8(line.content()).unwrap_or("");
        let line_prefix = match line.origin() {
            DiffLineType::Addition => "+",
            DiffLineType::Deletion => "-",
            _ => " ",
        };
        println!("{}{}", line_prefix, line_content);
        true
    }).expect("Failed to print diff");

    Ok(())
}

#[pymodule]
fn fugitrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_git_diff, m)?)?;

    Ok(())
}
