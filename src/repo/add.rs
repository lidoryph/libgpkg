use std::io::Error;
use std::path::Path;

pub enum AddRepoResult {
    FileDoesNotExist,
    CouldNotCheckFile(Error),
    NotValidRepo(sqlite::Error)
}

pub fn add_repo_file(repo: String) -> Result<(), AddRepoResult> {
    let repo_path = Path::new(&repo);

    match repo_path.try_exists() {
        Ok(true) => {}
        Ok(false) => { return Err(AddRepoResult::FileDoesNotExist) }
        Err(err) => { return Err(AddRepoResult::CouldNotCheckFile(err))}
    }

    let file_connection = sqlite::open(repo_path).unwrap();
    let valid_repo_file = file_connection.prepare("SELECT * FROM repo_info");

    if valid_repo_file.is_err() {
        return Err(AddRepoResult::NotValidRepo(valid_repo_file.err().unwrap()));
    }

    let repo_name = file_connection.prepare("SELECT name FROM repo_info").unwrap();
    

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
            Err(AddRepoResult::FileDoesNotExist)
        ));
    }

    #[test]
    pub fn not_valid_repo() {
        _ = File::create("not_valid_repo.repo");

        assert!(matches!(
            add_repo_file("not_valid_repo.repo".to_string()),
            Err(AddRepoResult::NotValidRepo(..))
        ));

        fs::remove_file("not_valid_repo.repo").unwrap();
    }
}