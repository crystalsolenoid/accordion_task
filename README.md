# Accordion Task Scheduler

## About

A personal routine scheduler with flexible timing. If something goes wrong, the scheduler should be able to adjust the rest of the task durations to accomodate.

Written in Rust with [Ratatui](https://ratatui.rs/). A TUI was chosen to keep things distraction-free and hopefully low-latency, as these are essential features.

## To Run

Routines are stored as portable, human-editable CSV files. The format of these routine files is not yet stable.

```
cargo run examples/test
```

...or make your own CSV routine file using `examples/test` as a template.

## Controls

- Press `enter` to check off (or uncheck) the selected task. Checking a task off will move on to the next task.
- Press `j` and `k` to go up and down. This allows tasks to be completed out of order, if life happens.
- Press `d` to toggle the debug panel.
- Press `q` or `esc` to quit.

## Planned Features

- [ ] Shrink the duration of each remaining task when behind schedule.
- [ ] Display how much the routine is behind or ahead of schedule.
- [ ] Allow running a routine with a target end time.
- [ ] Show a progress bar with relative durations and progress of each task. (Maybe with [tui-widget-list](https://github.com/preiter93/tui-widget-list)?)
- [ ] Generate log files for each routine session with data about the time taken and order of tasks.
- [ ] Allow pausing (but the main routine timer still has to run: I can't freeze time for you in real life!)
- [ ] Maybe someday: allow subtasks.

I am making this mostly for myself. I am prioritizing what I need. But I would like to share too, so I am planning on eventually working on things like configuration.

- [ ] Add an on-screen help widget with control reminders.
- [ ] Add better error messages and prevent panics and weird little edge cases.
- [ ] Use [AccessKit](https://github.com/AccessKit/accesskit) to make a more accessible interface. (Is that possible?)
- [ ] Allow user configuration (controls, ui style and compactness, whatever else)
