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

### Custom Deadline

By default, the deadline is set to the time you would complete all the tasks in the routine by if you took exactly as much time as specified in the routine CSV file.

It is possible to specify a deadline instead with a command flag:

```
cargo run examples/test -d 13:45
```

The deadline is assumed to be for today, unless the time has already past upon starting. If so, it is assumed to be for tomorrow.

## Controls

- Press `enter` to check off (or uncheck) the selected task. Checking a task off will move on to the next task.
- Press `s` to skip (or unskip) the selected task. This will add its budgeted time back to the pool without claiming it was completed.
- Press `j` and `k` to go up and down. This allows tasks to be completed out of order, if life happens.
- Press `J` and `K` to go up and down, skipping already-completed tasks.
- Press `p` to pause. Optionally, type a message before pressing `enter` to unpause to put it in the routine log.
- Press `i` to insert a task directly after the current task.
- Press `a` to append a task to the end of the routine.
- When creating a new task by either insert or append, press `esc` to cancel or `enter` to submit. After the name of the task and a space, a time may be specified like `12m30s`.
- Press `d` to toggle the debug panel.
- Press `ctrl + c` to quit.

## Config File

It is possible to create a config file to customize some behavior. It uses the [TOML format](https://toml.io/).

The config file will slowly grow as development progresses and as users weigh in. I haven't decided yet how to handle documenting the options. For now, please refer to the docstrings in `src/config.rs`.

## Planned Features

- [x] Shrink the duration of each remaining task when behind schedule.
- [ ] Display how much the routine is behind or ahead of schedule.
- [x] Allow running a routine with a target end time.
- [x] Allow the marking of a task as skipped but not completed, so that its duration contracts without disturbing statistics.
- [ ] Show a progress bar with relative durations and progress of each task. (Maybe with [tui-widget-list](https://github.com/preiter93/tui-widget-list)?)
- [x] Generate log files for each routine session with data about the time taken and order of tasks.
- [x] Allow pausing (but the main routine timer still has to run: I can't freeze time for you in real life!)
- [ ] Record mode: record a routine and save it and the observed timings to a routine file.
- [ ] Maybe someday: allow subtasks.

I am making this mostly for myself. I am prioritizing what I need. But I would like to share too, so I am planning on eventually working on things like configuration.

- [x] Add an on-screen help widget with control reminders.
- [ ] Add better error messages and prevent panics and weird little edge cases.
- [ ] Use [AccessKit](https://github.com/AccessKit/accesskit) to make a more accessible interface. (Is that possible?)
- [ ] Allow user configuration (controls, ui style and compactness, whatever else)
- [ ] Make a webapp version using a Rust framework (in the same repository; if it's more fun than it is trouble!)
