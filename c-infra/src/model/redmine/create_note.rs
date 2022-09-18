use c_domain::Note;

use derive_new::new;
use serde::Serialize;

#[derive(new, Serialize)]
pub struct CreateNoteParam {
    notes: String,
}

impl From<Note> for CreateNoteParam {
    fn from(note: Note) -> Self {
        Self::new(note.contents)
    }
}

#[derive(new, Serialize)]
pub struct CreateNote {
    issue: CreateNoteParam,
}
