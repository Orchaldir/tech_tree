#[derive(Clone, Debug, PartialEq)]
pub enum TechnologyName {
    Simple(String),
    Ranked {
        base: String,
        rank: u8,
        full: String,
    },
}

impl TechnologyName {
    pub fn new_ranked<S: Into<String>>(base: S, rank: u8) -> Self {
        let base = base.into();
        let full = format!("{} {}", base, rank);

        Self::Ranked { base, rank, full }
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
    fn test_get_full() {
        assert_eq!(TechnologyName::Simple("Test".to_string()).get_full(), "Test");
        assert_eq!(TechnologyName::new_ranked("Tech", 4).get_full(), "Tech 4");
    }
}
