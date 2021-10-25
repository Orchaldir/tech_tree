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

    pub fn new_simple<S: Into<String>>(base: S) -> Result<Self, AddError> {
        Ok(Self::Simple(Self::trim(&base.into())?))
    }

    pub fn new_ranked<S: Into<String>>(base: S, rank: u8) -> Result<Self, AddError> {
        let trimmed = Self::trim(&base.into())?;
        let full = format!("{} {}", trimmed, rank);

        Ok(Self::Ranked {
            base: trimmed,
            rank,
            full,
        })
    }

    fn trim(base: &str) -> Result<String, AddError> {
        let trimmed = base.trim();

        if trimmed.is_empty() {
            return Err(AddError::InvalidName(base.to_string()));
        }

        Ok(trimmed.to_string())
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
    fn test_new_simple() {
        assert_eq!(
            TechnologyName::new_simple("  Test  "),
            Ok(TechnologyName::Simple("Test".to_string()))
        );
    }

    #[test]
    fn test_new_simple_with_empty_string() {
        let string = "   ";
        assert_eq!(
            TechnologyName::new_simple(string).unwrap_err(),
            AddError::InvalidName(string.to_string())
        );
    }

    #[test]
    fn test_new_ranked() {
        assert_eq!(
            TechnologyName::new_ranked("  Tech  ", 4),
            Ok(TechnologyName::Ranked {
                base: "Tech".to_string(),
                rank: 4,
                full: "Tech 4".to_string()
            })
        );
    }

    #[test]
    fn test_new_ranked_with_empty_string() {
        assert_eq!(
            TechnologyName::new_ranked("", 4).unwrap_err(),
            AddError::InvalidName("".to_string())
        );
    }

    #[test]
    fn test_get_full_simple() {
        assert_eq!(
            TechnologyName::new_simple("  UVW   ").unwrap().get_full(),
            "UVW"
        );
    }

    #[test]
    fn test_get_full_ranked() {
        assert_eq!(
            TechnologyName::new_ranked("  ABC  ", 2).unwrap().get_full(),
            "ABC 2"
        );
    }
}
