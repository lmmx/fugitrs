use git2::{DiffFormat, Repository};
use pyo3::prelude::*;

#[pyfunction]
fn get_git_diff(repo_path: String, plain: Option<bool>) -> PyResult<()> {
    let repo = Repository::open(repo_path).expect("Failed to open repository");

    // Get the tree of the current HEAD commit
    let head_commit = repo
        .head()
        .and_then(|head| head.resolve())
        .and_then(|head| head.peel_to_commit())
        .expect("Failed to find head commit");
    let head_tree = head_commit.tree().expect("Failed to get head tree");

    // Perform a diff between the HEAD tree and the working directory
    let diff = repo
        .diff_tree_to_workdir_with_index(Some(&head_tree), None)
        .expect("Failed to create diff");

    // Check if plain output is requested
    let is_plain = plain.unwrap_or(false);

    // Print the diff with line prefixes and optional color
    diff.print(DiffFormat::Patch, move |_delta, _hunk, line| {
        let line_content = std::str::from_utf8(line.content()).unwrap_or("");
        let (line_prefix, maybe_color_code, maybe_reset_code) = match line.origin() {
            '+' => {
                if is_plain {
                    ("+", "\x1b[32m", "\x1b[0m") // Green color for added lines
                } else {
                    ("+", "", "")
                }
            }
            '-' => {
                if is_plain {
                    ("-", "\x1b[31m", "\x1b[0m") // Red color for deleted lines
                } else {
                    ("-", "", "")
                }
            }
            _ => (" ", "", ""),
        };
        print!(
            "{}{}{}{}",
            maybe_color_code, line_prefix, line_content, maybe_reset_code
        );
        true
    })
    .expect("Failed to print diff");

    Ok(())
}

#[pymodule]
fn fugitrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_git_diff, m)?)?;
    Ok(())
}
