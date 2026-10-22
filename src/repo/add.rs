use std::io::Error;
use std::path::Path;
use rusqlite::{Connection, OpenFlags};
use crate::repo::Repo;

static EXPECTED_COLUMNS: [&str; 5] = ["name", "description", "maintainers", "version", "url"];

pub enum AddRepoResult {
    FileDoesNotExist(String),
    CouldNotCheckFile(Error),
    CouldNotOpenRepo(rusqlite::Error),
    CouldNotReadDatabase(rusqlite::Error),
    NotValidRepo(String)
}

pub fn add_repo_file(repo_name: String) -> Result<(), AddRepoResult> {
    let repo: Repo;
    let repo_path = Path::new(&repo_name);

    match repo_path.try_exists() {
        Ok(true) => {}
        Ok(false) => { return Err(AddRepoResult::FileDoesNotExist(repo_name)) }
        Err(err) => { return Err(AddRepoResult::CouldNotCheckFile(err))}
    }

    let conn = Connection::open_with_flags(repo_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map(AddRepoResult::CouldNotOpenRepo)?;

    let info_exists = conn.table_exists(None, "repo_info")
        .map(AddRepoResult::CouldNotReadDatabase)?;

    if !info_exists {
        return Err(AddRepoResult::NotValidRepo("RepoInfoDoesNotExist".to_string()));
    }

    let repo_info = conn.prepare("SELECT * FROM repo_info")
        .map(AddRepoResult::CouldNotReadDatabase)?;

    let file_columns = repo_info.column_names();

    //TODO: Check column types and gather information for Repo struct

    for column in 1..6 {
        if file_columns[column] != EXPECTED_COLUMNS[column] {
            return Err(AddRepoResult::NotValidRepo("NotValidStructure".to_string()))
        }
    }



    Ok(())
}


#[cfg(test)]
pub mod tests{
    use std::fs;
    use std::fs::File;
    use super::*;
    #[test]
    pub fn unexistent_file() {
        assert!(matches!(
            add_repo_file("unexistent_repo_file.db".to_string()),
            Err(AddRepoResult::FileDoesNotExist(..))
        ));
    }

    #[test]
    pub fn not_valid_repo() {
        _ = File::create("not_valid_repo.repo");

        assert!(matches!(
            add_repo_file("not_valid_repo.repo".to_string()),
            Err(AddRepoResult::CouldNotOpenRepo(..))
        ));

        fs::remove_file("not_valid_repo.repo").unwrap();
    }
}