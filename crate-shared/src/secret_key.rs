use std::path::PathBuf;

pub struct SecretKey(pub PathBuf);

impl SecretKey {
    pub fn new(path: Option<String>) -> Option<Self> {
        path.or_else(|| Some("/key.pem".to_string()))
            .map(PathBuf::from)
            .filter(|path| path.exists() && path.is_file())
            .map(Self)
    }
}
