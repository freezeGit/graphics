//! Internal functions that do not require application specific customizations.

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
    /// Run simulation logic if dialog is not open.
    pub(super) fn event_loop(&mut self, ctx: &Context) {
        self.msgs.clear(); // establish invariant: Belt and suspenders

        // Draw shapes and widgets on the canvas.
        // Collect all messages from widgets into self.msgs.
        self.canvas.canvas.render(ctx, &mut self.msgs);

        // If an open dialog has just been closed or is already NilDlg,
        // set or reset the active dialog to NilDlg and run simulation.
        if self
            .canvas
            .canvas
            .get_mut_dialog()
            // Open active dialog and return true when it is closed.
            .invoke_modal(ctx, &mut self.msgs)
        {
            self.canvas.canvas.set_dialog(Box::new(NilDlg)); // NilDlg is always closed.
            self.step_simulation(ctx);
        } else {
            // Keep simulation timer synchronized while dialog is open,
            // so simulation does not jump ahead when the dialog closes.
            self.sim_timer.resync();
        }
    }

    /// Executes the simulation logic.
    /// This method is not required for many programs. It is only needed
    /// in case a simulation is run.
    ///
    /// This method checks if the simulation timer indicates that it's time
    /// to run the next simulation step. If so, it advances the state of the
    /// simulation's world model by one step by calling [`TheWorld::advance`] and then
    /// updates the canvas to reflect the world’s new state by calling [`TheCanvas::update`].

    fn step_simulation(&mut self, ctx: &egui::Context) {
        if !self.sim_timer.is_running() {
            return;
        }

        if self.sim_timer.fast_forward() {
            self.batch_step();
            ctx.request_repaint();
        } else {
            let now = self.time_now(ctx);
            self.step_when_ready(ctx, now);
            ctx.request_repaint_after(self.conditional_duration(SIM_REPAINT_16MS, now));
        }
    }

    /// Get current time in seconds from start of app1.
    pub fn time_now(&self, ctx: &Context) -> f64 {
        ctx.input(|i| i.time)
    }

    fn batch_step(&mut self) {
        for i in 0..self.sim_timer.batch_size() {
            self.world.advance();
        }
        self.canvas.update(&self.world);
    }

    fn step_when_ready(&mut self, ctx: &egui::Context, now: f64) {
        if self.sim_timer.ready(now) {
            self.world.advance();
            self.canvas.update(&self.world);
        }
    }

    fn conditional_duration(&self, condition: bool, now: f64) -> std::time::Duration {
        if condition {
            std::time::Duration::from_millis(16)
        } else {
            std::time::Duration::from_secs_f64(self.sim_timer.remaining(now))
        }
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
} // end impl TheApp
