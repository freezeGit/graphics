//! ids used to identify widgets and dialogs

use ::gui_lib as gl;
use gl::{ButtonId, DragFloatDlgId, DragFloatId, MessageBoxDlgId};
use gui_lib::{TextEntryDlgId, MultiTextEntryDlgId, RadioBoxesDlgId};

// ---------- Widget ids

pub const BTN_SIM: ButtonId = ButtonId(1);
pub const BTN_NEW_SIM: ButtonId = ButtonId(2);
pub const BTN_BATCH: ButtonId = ButtonId(3);
pub const BTN_ABOUT: ButtonId = ButtonId(4);

// ---------- Dialog ids
pub const DLG_ABOUT: MessageBoxDlgId = MessageBoxDlgId(1);
pub const DLG_BAD_VALS: MessageBoxDlgId = MessageBoxDlgId(2);
pub const DLG_BAD_BATCH: MessageBoxDlgId = MessageBoxDlgId(3);
// ------------
pub const DLG_BATCH: TextEntryDlgId = TextEntryDlgId(1);
// ------------
pub const DLG_ENTER_SPECS: MultiTextEntryDlgId = MultiTextEntryDlgId(1);
// ----------
pub const DLG_SIM_STATE: RadioBoxesDlgId = RadioBoxesDlgId(1);
