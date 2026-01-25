use std::fmt;

pub struct SSEEvent {
    pub id: Option<String>,
    pub event: Option<String>,
    pub data: String,
    pub retry: Option<u32>,
}

impl SSEEvent {
    pub fn new(data: String) -> Self {
        Self {
            id: None,
            event: None,
            data,
            retry: None,
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_event(mut self, event: String) -> Self {
        self.event = Some(event);
        self
    }

    pub fn with_retry(mut self, retry: u32) -> Self {
        self.retry = Some(retry);
        self
    }
}

impl fmt::Display for SSEEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = &self.id {
            writeln!(f, "id: {}", id)?;
        }
        if let Some(event) = &self.event {
            writeln!(f, "event: {}", event)?;
        }
        if let Some(retry) = self.retry {
            writeln!(f, "retry: {}", retry)?;
        }
        for line in self.data.lines() {
            writeln!(f, "data: {}", line)?;
        }
        writeln!(f)
    }
}
