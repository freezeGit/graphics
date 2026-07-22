//! ids used to identify widgets and dialogs

use ::gui_lib as gl;
use gl::{ButtonId, DragFloatDlgId, DragFloatId, MessageBoxDlgId,};
use gui_lib::{MultiTextEntryDlgId, RadioBoxesDlgId};

// ---------- Widget ids

pub(crate) const BTN_SIM: ButtonId = ButtonId(1);
pub(crate) const BTN_NEW_SIM: ButtonId = ButtonId(2);
pub(crate) const BTN_ABOUT: ButtonId = ButtonId(3);

// ---------- Dialog ids
pub(crate) const DLG_ABOUT: MessageBoxDlgId = MessageBoxDlgId(1);

//pub(crate) const DLG_ENTER_PERSON: MultiTextEntryDlgId = MultiTextEntryDlgId(1);
pub(crate) const DLG_ENTER_SPECS: MultiTextEntryDlgId = MultiTextEntryDlgId(1);
pub(crate) const DLG_ENTER_VALUE: DragFloatDlgId = DragFloatDlgId(1);
pub(crate) const DLG_SIM_STATE: RadioBoxesDlgId = RadioBoxesDlgId(1);
