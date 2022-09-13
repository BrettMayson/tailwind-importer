#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Level {
    #[default]
    Info,
    Danger,
    Success,
    Warning,
}

impl Level {
    pub const fn text(&self) -> &'static str {
        match *self {
            Self::Info => "text-blue-700",
            Self::Danger => "text-red-700",
            Self::Success => "text-green-700",
            Self::Warning => "text-yellow-700",
        }
    }

    pub const fn bg(&self) -> &'static str {
        match *self {
            Self::Info => "bg-blue-100",
            Self::Danger => "bg-red-100",
            Self::Success => "bg-green-100",
            Self::Warning => "bg-yellow-100",
        }
    }

    pub const fn border(&self) -> &'static str {
        match *self {
            Self::Info => "border-blue-500",
            Self::Danger => "border-red-500",
            Self::Success => "border-green-500",
            Self::Warning => "border-yellow-500",
        }
    }
}
