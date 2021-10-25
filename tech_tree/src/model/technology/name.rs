use crate::model::error::AddError;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum TechnologyName {
    Simple(String),
    Ranked {
        base: String,
        rank: u8,
        full: String,
    },
}

impl TechnologyName {
    pub fn new<S: Into<String>>(base: S) -> Result<Self, AddError> {
        let base = base.into();
        let parts: Vec<&str> = base.split_whitespace().collect();

        if parts.len() > 1 {
            if let Some((last, elements)) = parts.split_last() {
                let base = elements.join(" ");

                if let Ok(rank) = last.parse::<u8>() {
                    let full = format!("{} {}", base, rank);

                    return Ok(Self::Ranked { base, rank, full });
                }

                return Ok(Self::Simple(base));
            }
        } else if parts.is_empty() {
            return Err(AddError::InvalidName(base));
        }

        Ok(Self::Simple(parts.first().unwrap().to_string()))
    }

    pub fn get_full(&self) -> &str {
        match self {
            TechnologyName::Simple(name) => name,
            TechnologyName::Ranked { full, .. } => full,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_simple() {
        assert_eq!(
            TechnologyName::new("  Test  "),
            Ok(TechnologyName::Simple("Test".to_string()))
        );
    }

    #[test]
    fn test_new_with_ranked() {
        assert_eq!(
            TechnologyName::new("  Very  Advanced   Tech 4  "),
            Ok(TechnologyName::Ranked {
                base: "Very Advanced Tech".to_string(),
                rank: 4,
                full: "Very Advanced Tech 4".to_string()
            })
        );
    }

    #[test]
    fn test_new_with_empty_string() {
        let string = "   ";
        assert_eq!(
            TechnologyName::new(string).unwrap_err(),
            AddError::InvalidName(string.to_string())
        );
    }

    #[test]
    fn test_get_full_simple() {
        assert_eq!(TechnologyName::new("  UVW   ").unwrap().get_full(), "UVW");
    }

    #[test]
    fn test_get_full_ranked() {
        assert_eq!(
            TechnologyName::new("  ABC   2 ").unwrap().get_full(),
            "ABC 2"
        );
    }
}
