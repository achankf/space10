use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum MapIdIndicatorKind {
    #[default]
    Zone,
    Region,
    SuperRegion,
}

impl MapIdIndicatorKind {
    /// used as a name in HTML: lower-case, hyphen for space
    pub fn id_name(self) -> &'static str {
        match self {
            Self::Region => "region",
            Self::Zone => "zone",
            Self::SuperRegion => "super-region",
        }
    }
}

impl fmt::Display for MapIdIndicatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Region => "Region",
            Self::Zone => "Zone",
            Self::SuperRegion => "Super Region",
        };

        write!(f, "{value}")
    }
}
