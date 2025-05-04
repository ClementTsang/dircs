use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use console::{Term, truncate_str};
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};

pub(crate) struct ProgressBarState {
    main_progress: ProgressBar,
    thread_progress: Vec<(ProgressBar, AtomicBool)>,
    multi: MultiProgress,
    term: Term,
    started: AtomicBool,
}

impl Default for ProgressBarState {
    fn default() -> Self {
        let multi = MultiProgress::new();

        let main_progress = ProgressBar::hidden();
        main_progress.set_message("Starting...");
        main_progress
            .set_style(ProgressStyle::with_template("[{human_pos}/{human_len} files]").unwrap());
        main_progress.set_draw_target(ProgressDrawTarget::stderr());

        multi.add(main_progress.clone());

        let num_threads = rayon::current_num_threads();
        let thread_progress = (0..num_threads)
            .map(|_i| {
                let spinner = ProgressBar::new_spinner();

                multi.add(spinner.clone());

                (spinner, AtomicBool::new(false))
            })
            .collect();

        multi.set_move_cursor(true);

        Self {
            main_progress,
            thread_progress,
            multi,
            term: Term::stdout(),
            started: AtomicBool::new(false),
        }
    }
}

impl ProgressBarState {
    /// Update the length of the main progress bar by 1.
    pub(crate) fn update_length(&self) {
        self.main_progress.inc_length(1);

        let started = self.started.swap(true, Ordering::Relaxed);
        if !started {
            self.main_progress
                .enable_steady_tick(Duration::from_millis(100));

            for (tp, _) in &self.thread_progress {
                tp.enable_steady_tick(Duration::from_millis(100));
            }
        }
    }

    /// Update the progress of the main progress bar by 1.
    pub(crate) fn update_progress(&self) {
        self.main_progress.inc(1);
    }

    /// Mark the progress bar as done.
    pub(crate) fn finish(self) {
        for tp in self.thread_progress {
            tp.0.finish_and_clear();
        }

        self.main_progress.finish_and_clear();

        let _ = self.multi.clear();
    }

    /// Set thread progress as started for a specific thread index. If the index
    /// does not exist this function will silently ignore it.
    pub(crate) fn set_thread_progress(&self, index: usize, path: &Path) {
        if let Some((tp, lock)) = self.thread_progress.get(index) {
            // Idea is we swap in a true, if the value we get back is _already_
            // true, then it's in use and it was a no-op. If it's false, then
            // we are the owner!
            let is_used = lock.swap(true, Ordering::Relaxed);

            if !is_used {
                let (_, width) = self.term.size();
                let message = truncate_str(
                    &path.as_os_str().to_string_lossy(),
                    width.saturating_sub(10).into(), // Slight offset to account for spinners
                    "…",
                )
                .into_owned();
                tp.set_message(message);
            }
        }
    }

    /// Finish thread progress for a specific thread index. If the index
    /// does not exist this function will silently ignore it.
    pub(crate) fn finish_thread_progress(&self, index: usize) {
        if let Some((tp, lock)) = self.thread_progress.get(index) {
            lock.store(false, Ordering::Relaxed);
            tp.set_message("");
        }
    }
}
