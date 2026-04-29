use godot::prelude::*;
use std::fmt::Display;

use crate::year_iterator::YearIterator;

#[derive(Debug, Clone, GodotClass)]
#[class(no_init)]
pub struct Year {
    #[var(no_set)]
    year: i32,
}

impl Year {
    fn current() -> Option<Self> {
        Some(Self {
            year: temporal_rs::Temporal::utc_now()
                .plain_date_iso(None)
                .ok()?
                .year(),
        })
    }

    pub fn get(&self) -> i32 {
        self.year
    }
}

#[godot_api]
impl Year {
    #[func]
    fn label(&self) -> GString {
        self.to_string().to_godot()
    }

    #[func]
    fn get_current() -> Option<Gd<Self>> {
        Self::current().map(Gd::from_object)
    }

    #[func]
    fn get_next(&self) -> Option<Gd<Self>> {
        let mut temp_iter = YearIterator::new(self)?;
        temp_iter.nth(1).map(Gd::from_object)
    }

    #[func]
    fn get_previous(&self) -> Option<Gd<Self>> {
        let mut temp_iter = YearIterator::new(self)?;
        temp_iter.nth_back(1).map(Gd::from_object)
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.year)
    }
}

pub trait ToYear {
    fn to_year(&self) -> Year;
}

impl ToYear for temporal_rs::PlainDate {
    fn to_year(&self) -> Year {
        Year { year: self.year() }
    }
}
