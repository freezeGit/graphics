//! ids used to identify widgets and dialogs

use ::gui_lib as gl;
use gl::{ButtonId, DragFloatDlgId, DragFloatId, MessageBoxDlgId, SliderId, TextEntryDlgId};
use gui_lib::MultiTextEntryDlgId;

// ---------- Widget ids
pub(crate) const SLIDER_GAUGE: SliderId = SliderId(1);
pub(crate) const SLIDER_ANOTHER: SliderId = SliderId(2);

pub(crate) const DRAGFLOAT_GAUGE: DragFloatId = DragFloatId(1);

pub(crate) const BTN_STATE_A: ButtonId = ButtonId(1);
pub(crate) const BTN_STATE_B: ButtonId = ButtonId(2);
pub(crate) const BTN_RUN_PAUSE: ButtonId = ButtonId(3);
pub(crate) const BTN_ABOUT: ButtonId = ButtonId(4);
pub(crate) const BTN_ENTER_NAME: ButtonId = ButtonId(5);
pub(crate) const BTN_PERSON: ButtonId = ButtonId(6);
//pub(crate) const BTN_PERSON: ButtonId = ButtonId(9);
pub(crate) const BTN_ENTER_VALUE: ButtonId = ButtonId(7);

// ---------- Dialog ids
pub(crate) const DLG_ABOUT: MessageBoxDlgId = MessageBoxDlgId(1);
pub(crate) const DLG_ENTER_NAME: TextEntryDlgId = TextEntryDlgId(1);
//pub(crate) const DLG_ENTER_PERSON: TextEntryDlgId = TextEntryDlgId(1);
pub(crate) const DLG_ENTER_PERSON: MultiTextEntryDlgId = MultiTextEntryDlgId(1);
pub(crate) const DLG_ENTER_VALUE: DragFloatDlgId = DragFloatDlgId(1);
