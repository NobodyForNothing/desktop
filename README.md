# Apps

Small desktop and android Projects that wouldn't be noteworthy otherwise. The goal is to implement components so that they can be considered "completed" and require no further maintenance (except bumping framework/library versions).

## Overview

The following section outlines current apps and libraries and their purpose. Some `rust/` and `packages/` libraries are left out as they are not useful enough on their own.

### Completed

- `rust/dir_size` - A tool to recursively determine the size of a directory
- `rust/iniconf` - A library to parse config filea in ini format

### Nearly completed

- `apps/calculator` - A kalker based calculator app, lacks android support
- `apps/rewordle` - Reimplementation of the classical world game, lacks some minor fidelity

### Ongoing

- `apps/file_manager` - Material design graphical file manager for desktop
- `rust/vcs` - custom minimal git implementation

# Building

While there is some effort to write a tool/script set that can do everything there are currently 3 processes:

- melos for testing and linting flutter/dart projects and unifying dependencies
- manual build for flutter app releases
- normal rust build for rust apps
