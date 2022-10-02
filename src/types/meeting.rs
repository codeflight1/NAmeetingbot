use chrono::{DateTime, Duration, Utc};
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum RepeatType {
    Weekly,
    BiWeekly,
    Monthly,
    None,
}

#[derive(Clone)]
pub struct Meeting {
    pub name: String,
    pub first_date: DateTime<Utc>,
    pub next_date: DateTime<Utc>,
    pub repeat: RepeatType,
}

pub enum ErrType {
    RepeatTypeNone,
}

impl Meeting {
    pub fn propogate_next(&mut self) -> Result<(), ErrType> {
        self.next_date = match self.repeat {
            RepeatType::Weekly => self.next_date + Duration::weeks(1),
            RepeatType::BiWeekly => self.next_date + Duration::weeks(2),
            RepeatType::Monthly => self.next_date + Duration::weeks(4),
            RepeatType::None => {
                return Err(ErrType::RepeatTypeNone);
            }
        };

        Ok(())
    }
}

impl fmt::Display for Meeting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = write!(
            f,
            "**{0}:**\nLocal Time: <t:{1}:F> (<t:{1}:R>)\nUTC Time: {2}",
            self.name,
            self.next_date.timestamp(),
            self.next_date.format("%A, %B %d, %Y %H:%M")
        );

        if self.repeat != RepeatType::None {
            result = write!(f, "\nRepeat Type: {:?}", self.repeat);
        }

        result
    }
}
