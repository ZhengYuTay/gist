extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct GistFile {
    #[serde(skip_serializing, default = "GistFile::default_name")]
    pub name: String,
    #[serde(rename(serialize = "content"))]
    pub content: String,
}

impl GistFile {
    pub fn new(name: String, content: String) -> GistFile {
        GistFile {
            name: name,
            content: content,
        }
    }

    pub fn default_name() -> String {
        String::from("content")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
