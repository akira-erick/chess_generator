use shakmaty::Move;

pub struct HistoryEntry {
    pub move_number: u32,
    pub player: String,
    pub played_move: Move,
}

pub struct History {
    entries: Vec<HistoryEntry>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: HistoryEntry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }
}