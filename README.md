Podocpodoc
==========
Podocpodoc is a text editor. Podocpodoc was started as a forked version from [hecto-tutorial](https://github.com/pflenker/hecto-tutorial/tree/main)

> podocpodoc(포닥포닥) in Korean means that the sound of a small bird beating its wings in light, quick succession. Or the shape of it.

## Usage

Podocpodoc uses helix like key bindings.

### Normal mode

Normal mode is the default mode. You can return to it from other modes by pressing the Escape key

#### Commands

| name | description |
| ---- | ----------- |
| :q   | close the current view |
| :q!  | Force close the current view, ignoring unsaved changes |
| :w   | Write changes to disk |
| :wq  | Write changes to disk and close the current view |

#### Movement

| key | description |
| --- | ----------- |
| Ctrl-b | Move page up |
| Ctrl-f | Move page down |
| Ctrl-u | Move cursor and page half page up |
| Ctrl-d | Move cursor and page half page down |
| Ctrl-e | scroll the window up one line |
| Ctrl-y | scroll the window down one line |

#### Changes

| key | description |
| --- | ----------- |
| i   | Insert before selection |
| a   | Insert after selection (append) |
| o   | Open new line below selection |
| O   | Open new line above selection |

### Insert mode

Accessed by typing i in normal mode.

| key | description |
| --- | ----------- |
