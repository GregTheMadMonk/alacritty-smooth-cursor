<p align="center">
    <img width="200" alt="Alacritty Logo" src="https://raw.githubusercontent.com/GregTheMadMonk/alacritty-smooth-cursor/refs/heads/patch/readme-base/extra/logo/alacritty-term%2Bscanlines.svg">
</p>

<h1 align="center">Alacritty - A fast, cross-platform, OpenGL terminal emulator</h1>

## About this fork!

This fork originally introduced only the smooth cursor motion, but now also
offers a second patch (**seltools**).
If you are only interested in one thing, all patches are independent and could
be built individually from __patch/...__ branches.

Package in AUR (contains all available patches):
https://aur.archlinux.org/packages/alacritty-smooth-cursor-git .

This branch contains the following patches:

### smooth-cursor

This fork introduces basic cursor animations to Alacritty in a rather simple way.
It replaces the block cursor render by a rectangle with a changed blend mode.

<p align="center">
    <img width="100%" alt="Alacritty smooth cursor demo" src="https://raw.githubusercontent.com/GregTheMadMonk/alacritty-smooth-cursor/refs/heads/patch/smooth-cursor/extra/demo.gif">
</p>

#### Known issues

* Block cursor may look really off due to blending hack that essentially just
  inverts the character color. It is recommended to replace it with "underline"
  (see config or GIF demo)
* __Wayland compatibility is fixed!__ If you still experience issues, report
  them to #3 and feel free to reopen it
* Probably more resource consumption when redrawing the frame, but overall the
  performance drawbacks should be __mostly mitigated__ by now
* Animation speed depending on FPS is __fixed__, you might want to adjust
  your config since, well, your FPS doesn't get factored in anymore :)

#### Configure

Add/change the following entries in your `alacritty.toml`:

```toml
[cursor]
# Set to false to disable completely
smooth_motion = true
# 0.0 = cursor is not moving, 1.0 = cursor moves instantly
smooth_motion_factor = 0.2
# 0.0 = broken, 1.0 = cursor shape is unaffected by movement
smooth_motion_spring = 0.5
# Limits how the cursor size may change
smooth_motion_max_stretch_x = 3.0
smooth_motion_max_stretch_y = 3.0
# Override "block" cursor if you don't like how it looks in this fork
# I prefer "underline"
block_replace_shape = "underline"
```
_Back to the original README..._

<hr>

<p align="center">
  <img alt="Alacritty - A fast, cross-platform, OpenGL terminal emulator"
       src="https://raw.githubusercontent.com/alacritty/alacritty/master/extra/promo/alacritty-readme.png">
</p>

## About

Alacritty is a modern terminal emulator that comes with sensible defaults, but
allows for extensive [configuration](#configuration). By integrating with other
applications, rather than reimplementing their functionality, it manages to
provide a flexible set of [features](./docs/features.md) with high performance.
The supported platforms currently consist of BSD, Linux, macOS and Windows.

The software is considered to be at a **beta** level of readiness; there are
a few missing features and bugs to be fixed, but it is already used by many as
a daily driver.

Precompiled binaries are available from the [GitHub releases page](https://github.com/alacritty/alacritty/releases).

Join [`#alacritty`] on libera.chat if you have questions or looking for a quick help.

[`#alacritty`]: https://web.libera.chat/gamja/?channels=#alacritty

## Features

You can find an overview over the features available in Alacritty [here](./docs/features.md).

## Further information

- [Announcing Alacritty, a GPU-Accelerated Terminal Emulator](https://jwilm.io/blog/announcing-alacritty/) January 6, 2017
- [A talk about Alacritty at the Rust Meetup January 2017](https://www.youtube.com/watch?v=qHOdYO3WUTk) January 19, 2017
- [Alacritty Lands Scrollback, Publishes Benchmarks](https://jwilm.io/blog/alacritty-lands-scrollback/) September 17, 2018

## Installation

Alacritty can be installed by using various package managers on Linux, BSD,
macOS and Windows.

Prebuilt binaries for macOS and Windows can also be downloaded from the
[GitHub releases page](https://github.com/alacritty/alacritty/releases).

For everyone else, the detailed instructions to install Alacritty can be found
[here](INSTALL.md).

### Requirements

- At least OpenGL ES 2.0
- [Windows] ConPTY support (Windows 10 version 1809 or higher)

## Configuration

You can find the documentation for Alacritty's configuration in `man 5
alacritty`, or by looking at [the website] if you do not have the manpages
installed.

[the website]: https://alacritty.org/config-alacritty.html

Alacritty doesn't create the config file for you, but it looks for one in the
following locations:

1. `$XDG_CONFIG_HOME/alacritty/alacritty.toml`
2. `$XDG_CONFIG_HOME/alacritty.toml`
3. `$HOME/.config/alacritty/alacritty.toml`
4. `$HOME/.alacritty.toml`
5. `/etc/alacritty/alacritty.toml`

On Windows, the config file will be looked for in:

* `%APPDATA%\alacritty\alacritty.toml`

## Contributing

A guideline about contributing to Alacritty can be found in the
[`CONTRIBUTING.md`](CONTRIBUTING.md) file.

## FAQ

**_Is it really the fastest terminal emulator?_**

Benchmarking terminal emulators is complicated. Alacritty uses
[vtebench](https://github.com/alacritty/vtebench) to quantify terminal emulator
throughput and manages to consistently score better than the competition using
it. If you have found an example where this is not the case, please report a
bug.

Other aspects like latency or framerate and frame consistency are more difficult
to quantify. Some terminal emulators also intentionally slow down to save
resources, which might be preferred by some users.

If you have doubts about Alacritty's performance or usability, the best way to
quantify terminal emulators is always to test them with **your** specific
usecases.

**_Why isn't feature X implemented?_**

Alacritty has many great features, but not every feature from every other
terminal. This could be for a number of reasons, but sometimes it's just not a
good fit for Alacritty. This means you won't find things like tabs or splits
(which are best left to a window manager or [terminal multiplexer][tmux]) nor
niceties like a GUI config editor.

[tmux]: https://github.com/tmux/tmux

## License

Alacritty is released under the [Apache License, Version 2.0].

[Apache License, Version 2.0]: https://github.com/alacritty/alacritty/blob/master/LICENSE-APACHE
