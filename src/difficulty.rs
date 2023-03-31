use std::fmt::Formatter;

#[derive(Copy, Clone, Debug)]
pub enum Difficulty {
    Hard = 30,
    Medium = 40,
    Easy = 50,
}

impl Difficulty {
    pub fn up(self) -> Self {
        match self {
            Difficulty::Hard   => Difficulty::Hard,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Easy   => Difficulty::Medium
        }
    }

    pub fn down(self) -> Self {
        match self {
            Difficulty::Hard   => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Easy,
            Difficulty::Easy   => Difficulty::Easy
        }
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",match *self {
            Difficulty::Hard => "Hard",
            Difficulty::Medium => "Medium",
            Difficulty::Easy => "Easy"
        })
    }
}
