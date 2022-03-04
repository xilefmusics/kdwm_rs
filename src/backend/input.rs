use crate::c;

#[derive(Debug, Clone)]
pub enum Input {
    SubstructureRedirect,
    SubstructureNotify,
    KeyPress,
}

impl Input {
    pub fn mask(&self) -> u32 {
        match self {
            Self::SubstructureRedirect => c::SubstructureRedirectMask,
            Self::SubstructureNotify => c::SubstructureNotifyMask,
            Self::KeyPress => c::KeyPressMask,
        }
    }
}
