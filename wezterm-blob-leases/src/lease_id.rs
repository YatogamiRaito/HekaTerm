use uuid::Uuid;

/// Represents an individual lease
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct LeaseId {
    uuid: Uuid,
    pid: u32,
}

impl std::fmt::Display for LeaseId {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "lease:pid={},{}", self.pid, self.uuid.hyphenated())
    }
}

impl Default for LeaseId {
    fn default() -> Self {
        Self::new()
    }
}

impl LeaseId {
    #[must_use]
    pub fn new() -> Self {
        let uuid = Uuid::new_v4();
        let pid = std::process::id();
        Self { uuid, pid }
    }

    #[must_use]
    pub const fn pid(&self) -> u32 {
        self.pid
    }
}
