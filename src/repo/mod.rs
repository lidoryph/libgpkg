pub mod add;

struct Repo {
    name: String,
    description: String,
    maintainers: String,
    version: String,
    url: String,
    sets: Vec<(String, bool)>
}