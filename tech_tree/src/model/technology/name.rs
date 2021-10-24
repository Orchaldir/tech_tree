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
    pub fn new_simple<S: Into<String>>(base: S) -> Result<Self, AddError> {
        let base = base.into();
        let trimmed = base.trim();

        if trimmed.is_empty() {
            return Err(AddError::InvalidName(base));
        }

        Ok(Self::Simple(trimmed.to_string()))
    }

    pub fn new_ranked<S: Into<String>>(base: S, rank: u8) -> Result<Self, AddError> {
        let base = base.into();
        let trimmed = base.trim();

        if trimmed.is_empty() {
            return Err(AddError::InvalidName(base));
        }

        let full = format!("{} {}", trimmed, rank);

        Ok(Self::Ranked { base, rank, full })
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
    fn test_simple() {
        assert_eq!(
            TechnologyName::new_simple("Test").unwrap().get_full(),
            "Test"
        );
    }

    #[test]
    fn test_simple_with_extra_whitespaces() {
        assert_eq!(
            TechnologyName::new_simple("  UVW   ").unwrap().get_full(),
            "UVW"
        );
    }

    #[test]
    fn test_simple_with_empty_string() {
        let string = "   ";
        assert_eq!(
            TechnologyName::new_simple(string).unwrap_err(),
            AddError::InvalidName(string.to_string())
        );
    }

    #[test]
    fn test_ranked() {
        assert_eq!(
            TechnologyName::new_ranked("Tech", 4).unwrap().get_full(),
            "Tech 4"
        );
    }

    #[test]
    fn test_ranked_with_extra_whitespaces() {
        assert_eq!(
            TechnologyName::new_ranked("  ABC  ", 2).unwrap().get_full(),
            "ABC 2"
        );
    }

    #[test]
    fn test_ranked_with_empty_string() {
        assert_eq!(
            TechnologyName::new_ranked("", 4).unwrap_err(),
            AddError::InvalidName("".to_string())
        );
    }
}
