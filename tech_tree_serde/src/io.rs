use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn read<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let string = fs::read_to_string(path).context(format!("Failed to read {:?}", path))?;
    let data: T = serde_yaml::from_str(&string).context(format!("Failed to parse {:?}", path))?;
    Ok(data)
}

pub fn write<T: Serialize>(object: &T, path: &Path) -> Result<()> {
    let mut file = File::create(path).context(format!("Failed to write to {:?}", path))?;
    let s = serde_yaml::to_string(object).context(format!("Failed to parse {:?}", path))?;

    file.write_all(s.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definition::technology::TechnologyDefinition;
    use tempfile::tempdir;

    #[test]
    fn test_io() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.yaml");
        let definition =
            TechnologyDefinition::new("t2".to_string(), vec!["t0".to_string(), "t1".to_string()]);

        write(&definition, &file_path).expect("Writing failed");

        let definition_from_file: TechnologyDefinition = read(&file_path).expect("Reading failed");

        assert_eq!(definition_from_file, definition)
    }
}
