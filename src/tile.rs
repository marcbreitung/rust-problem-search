#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
/// Defines the type of a single type
pub enum Tile {
    /// No value
    None = 0,
    /// Path (walkable)
    Path = 1,
    /// Ground (not walkable)
    Ground = 2,
}

impl Tile {
    /// Builds a type from a u8 value
    ///
    /// # Arguments
    ///
    /// * `value` - An u8 which defines the type
    ///
    /// # Example
    ///
    /// ```
    /// use crate::rust_problem_search::tile::Tile;
    ///
    /// let path = Tile::from_u8(1);
    /// ```
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Tile::Path,
            2 => Tile::Ground,
            _ => Tile::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_with_u8_1_returns_path() {
        assert_eq!(Tile::Path, Tile::from_u8(1));
    }

    #[test]
    fn from_with_u8_2_returns_ground() {
        assert_eq!(Tile::Ground, Tile::from_u8(2));
    }

    #[test]
    fn from_with_u8_0_returns_none() {
        assert_eq!(Tile::None, Tile::from_u8(0));
    }

    #[test]
    fn from_with_u8_9_returns_none() {
        assert_eq!(Tile::None, Tile::from_u8(9));
    }
}
