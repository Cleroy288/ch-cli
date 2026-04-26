use serde::Deserialize;

#[derive(Deserialize)]
pub struct BbPage<T> {
    pub values: Vec<T>,
}

#[derive(Deserialize)]
pub struct BbBranch {
    pub name: String,
}

#[derive(Deserialize)]
pub struct BbPullRequest {
    pub id: u64,
    pub title: String,
    /// OPEN, MERGED, or DECLINED
    pub state: String,
}
