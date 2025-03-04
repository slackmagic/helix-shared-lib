#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub version: String,
    pub version_name: String,
    pub git_version: GitVersion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitVersion {
    pub commit_short_hash: String,
    pub commit_message: String,
    pub commit_date: String,
}

impl Version {
    pub fn new(
        version: String,
        version_name: String,
        git_commit_short_hash: String,
        git_commit_message: String,
        git_commit_date: String,
    ) -> Version {
        Version {
            version: version,
            version_name: version_name,
            git_version: GitVersion {
                commit_message: git_commit_message,
                commit_short_hash: git_commit_short_hash,
                commit_date: git_commit_date,
            },
        }
    }
}
