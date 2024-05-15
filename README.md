# rustgrab
Screenshot Utility made in Rust for ex-ShareX Users.

**Note:** This is a fork of [ShareXin](https://github.com/sharexin/sharexin)

# Table of contents
* [Requirements](#requirements)
* [Features](#features)
* [Desktop support](#desktop-support)
    * [Tested on](#tested-on)
* [How to Use](#how-to-use)
* [Language support](#language-support)
* [Changelog](#changelog)

## Requirements
* `rustc` 1.78.0 (Rust 2021)
* `feh` (only need if `spectacle` is not installed or using GNOME)
* `xclip` for copying non-png files.

## Features
* Takes screenshots
* Uploads to ~~Twitter, Mastodon~~, XBackbone, and Imgur
* Saves screenshots to your Pictures
* Notifications
* ~~GTK Dialog for entering a message with a tweet or toot~~ Planning to be re-written with Qt.

## Desktop support
- GNOME desktop **(with `gnome-screenshot`)**
- KDE Plasma desktop **(with `spectacle`)**
- Budgie desktop
- Cinnamon desktop
- Unity desktop
- Generic X11 DE **(with `scrot`)**

Screenshot handling is done with [screenshot-rs](https://crates.io/crates/screenshot-rs).
#### Tested on
- Debian Trixie w/ KDE 5 (2024/05/12)

## How to Use
Currently pending a rewrite.

## Language support
* English
* Français (French) by [@Eleoryth](https://twitter.com/Eleoryth)
* Español (Spanish)
* Esperanto
* 简体中文 (Simplified Chinese)
* 繁體中文 (Traditional Chinese)
* 日本語 (Japanese)
*  한국어 (Korean)
* Deutsch (German) by [@qwertxzy](https://twitter.com/qwertxzy)
* Polski (Polish) by [@Michcioperz](https://twitter.com/Michcioperz)
* Português (Portuguese) by [@pillgp](https://twitter.com/pillgp)

# Changelog

#### [0.7.4] - 2024-05-15
- Remove support for Twitter and Mastodon.
- Drop gtk/gdk/dio dependency.
- Add support for post-upload actions.
- **Note** manual file will be out-of-date until the codebase refactor is complete (est. v0.8 maybe)

#### [0.7.3] - 2024-05-12
- Update dependencies
- Flatpak/Application ID renamed to pet.kate.rustgrab (from io.github.ShareXin)
- Renamed ShareXin to rustgrab
- **[Forked by ktwrd](https://github.com/ktwrd/rustgrab)**


**Old changelog content can be found in [Changelog.md](Changelog)**