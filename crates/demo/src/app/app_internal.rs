// app_internal.rs

use super::*;
use gui_lib::World;
//use egui::Context;

// --------- Helper functions for App::update() --------------------------

impl TheApp {
    /// Establish event loop.
    ///
    /// Render canvas and collect any emitted widgets messages in [`Self::msgs`].
    /// Invoke active dialog and collect emitted message in [`Self::msgs`].
    /// Run simulation logic if dialog is not open (if program includes a simulation)
    pub(super) fn event_loop(&mut self, ctx: &Context) {
        self.msgs.clear(); // establish invariant: Belt and suspenders

        // Draw shapes and widgets on the canvas.
        // Collect all messages from widgets into self.msgs.
        self.canvas.canvas.render(ctx, &mut self.msgs);
        // self.canvas.update(&self.world); // DDJ:

        if self.active_dialog_closed(ctx) {
            // NilDlg is always closed.
            self.canvas.canvas.set_dialog(Box::new(NilDlg));
            // Simulation is only run if the dialog is not open.
            self.run_simulation(ctx);
        } else {
            self.sim_timer.resync();
        }

        // // Invoke active dialog.
        // // When the dialog is closed push its message into self.msgs.
        // // Pause simulation while dialog is open.
        // if self.dialog_invoked_and_closed(ctx) {
        //     // If the active dialog has been closed, set the dialog to nil
        //     self.canvas.canvas.set_dialog(Box::new(NilDlg));
        //     // // Only run simulation (if one exists) if the dialog is not open.
        //     self.run_simulation(ctx);
        // }

        //self.canvas.update(&self.world); // DDJ:xxx
    }


    /// Calls [`Dialog::invoke_modal`] to draw and get a message from a modal dialog.
    ///
    /// Parameter `ctx`: A reference to the [`Context`] object.
    ///
    /// Returns `true` if the user has closed the dialog,
    /// or `false` if the dialog is still open.
    fn active_dialog_closed(&mut self, ctx: &Context) -> bool {
        self.canvas
            .canvas
            .get_mut_dialog()
            .invoke_modal(ctx, &mut self.msgs)
    }

    /// Executes the simulation logic.
    /// This method is not required for many programs. It is only needed
    /// in case a simulation is run.
    ///
    /// This method checks if the simulation timer indicates that it's time
    /// to run the next simulation step. If so, it advances the state of the
    /// simulation's world model by one step by calling [`TheWorld::advance`] and then
    /// updates the canvas to reflect the world’s new state by calling [`TheCanvas::update`].
    ///
    /// Parameter `ctx`: A reference to the [`Context`] object.
    fn run_simulation(&mut self, ctx: &egui::Context) {
        if !self.sim_timer.is_running() {
            return;
        }

        let now = ctx.input(|i| i.time);

        if self.sim_timer.ready(now) {
            self.world.advance();
            self.canvas.update(&self.world);
        }

        ctx.request_repaint_after(std::time::Duration::from_secs_f64(
            self.sim_timer.remaining(now),
        ));

        //ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }

    //  --------- Handle messages if any exist---------------------

    /// Handle messages if any exist
    /// # Related Methods
    /// - [`handle_msg`]: Called for each individual message in the `msgs` buffer.
    /// - [`canvas.update`]: Updates the canvas to reflect changes in the `world`.
    pub(super) fn handle_emitted_messages(&mut self) {
        // Handle messages if any exist
        if !self.msgs.is_empty() {
            // Move msgs out of self so we can mutably borrow self inside the loop.
            let mut msgs = std::mem::take(&mut self.msgs);
            // Handle messages and drain the buffer.
            for msg in msgs.drain(..) {
                self.handle_msg(msg);
            }
            // Put the buffer back (empty, but keeps its capacity).
            self.msgs = msgs;

            // Update canvas to reflect all state changes:
            self.canvas.update(&self.world);
        }
    }

    /// What to do with [`WidgetMsg`] messages from widgets and dialogs.
    /// This is the only communication between the GUI and the program code.
    /// Program data and logic are encapsulated in struct [`TheWorld`].
    fn handle_msg(&mut self, msg: WidgetMsg) {
        match msg {
            WidgetMsg::ButtonClicked(id) => {
                self.handle_button(id);
            }
            WidgetMsg::DragFloatChanged(id, value) => {
                self.handle_drag_float(id, value);
            }
            // WidgetMsg::DialogAcceptedText(id, text) => {
            //     self.handle_text_entry(id, text);
            // }
            WidgetMsg::DialogAcceptedMultiTextEntry(id, values) => {
                self.handle_multi_text_entry(id, values);
            }
            WidgetMsg::DialogAcceptedDragFloat(id, val) => {
                self.handle_drag_float_dlg(id, val);
            }
            _ => {}
        }
    }
} // end impl TheApp
