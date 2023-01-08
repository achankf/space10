use crate::TileKind;

impl TileKind {
    pub fn get_color(self) -> &'static str {
        match self {
            Self::Desert => "rgb(242, 210, 169, 0.2)",
            Self::Forest => "rgb(34, 139, 34, 0.2)",
            Self::Hill => "rgb(155, 118, 83, 0.2)",
            Self::Mountain => "rgb(90,77,65,0.2)",
            Self::Ocean => "rgb(43,101,236,0.2)",
            Self::Plain => "rgb(50,205,50,0.2)",
            Self::Unknown => "rgb(0,0,0,1)",
        }
    }
}
