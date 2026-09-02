//! ids used to identify widgets and dialogs

use ::gui_lib as gl;
use gl::{ButtonId, DragFloatDlgId, DragFloatId, MessageBoxDlgId};
use gui_lib::{MultiTextEntryDlgId, RadioBoxesDlgId, TextEntryDlgId};

// ---------- Widget ids
pub const BTN_DELTAS: ButtonId = ButtonId(1);

pub const BTN_ABOUT: ButtonId = ButtonId(2);

// ---------- Dialog ids
pub const DLG_ABOUT: MessageBoxDlgId = MessageBoxDlgId(1);

pub const DLG_BAD_VALS: MessageBoxDlgId = MessageBoxDlgId(2);
pub const DLG_BAD_BATCH: MessageBoxDlgId = MessageBoxDlgId(3);

pub const DLG_ENTER_SPECS: MultiTextEntryDlgId = MultiTextEntryDlgId(1);
