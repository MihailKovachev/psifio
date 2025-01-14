#[derive(Debug, Clone, Copy)]
pub struct HandTotal {
    value: u8,
    is_soft: bool
}

impl HandTotal {
    pub fn new(value: u8, is_soft: bool) -> Self {
        Self {
            value,
            is_soft
        }
    }
    
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn is_soft(&self) -> bool {
        self.is_soft
    }
}

impl core::fmt::Display for HandTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.is_soft {
            write!(f, "Hard {}", self.value)
        } else {
            write!(f, "Soft {}", self.value)
        }
    }
}

pub trait HandTotalable {
    fn total(&self) -> HandTotal;
}