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
* Uploads to ~~Twitter, Mastodon~~, XBackbone, Google Cloud Storage, and Imgur
* Saves screenshots to your Pictures
* Notifications
* ~~GTK Dialog for entering a message with a tweet or toot~~ Planning to be re-written with [fltk-rs](https://crates.io/crates/fltk).

## Desktop support
- GNOME desktop **(with `gnome-screenshot`)**
- KDE Plasma desktop **(with `spectacle`)**
- Budgie desktop
- Cinnamon desktop
- Unity desktop
- Generic X11 DE **(with `scrot`)**

Screenshot handling is done with [screenshot-rs](https://crates.io/crates/screenshot-rs).
#### Tested on
- Debian Trixie w/ KDE 5 (2024/05/16)

## How to Use

### Google Cloud Storage
To use the Google Cloud Storage target, you must have the following;
- `gcloud` CLI installed
- An existing Google Cloud Storage Bucket

If you wish to not use a service account, you can easily create credentials with `gcloud auth application-default login`, which will create credentials for the current project that you're authenticated as.

Once you've done that, set your `gcs_config` property in your config to the following;
```json
{
    "use_default": false,
    "auth_cfg_location": "/home/myuser/.config/gcloud/application_default_credentials.json",
    "bucket": "my_bucket_name",
    "relative_path": "upload/$rand",
    "public_url_base": null
}
```

Once you've modified your config, you can now change your default target to `GoogleCloudStorage` so rustgrab will upload to your Google Cloud Storage bucket by default.

**Note** `public_base_url` will default to `https://storage.googleapis.com/$bucket_name` where `$bucket_name` is the value of `bucket` in `gcs_config`.

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

When changes have been made to the locale files, run `generate_include.js` in the locale folder. This will generate `src/locale_content.rs`.

# Changelog
#### [0.8] - 2024-06-05
- Add GUI for editing `~/.config/rustgrab/config.json`. Made with [fltk-rs](https://crates.io/crates/fltk).
- Add ability to write to the config file.
- Remove dependency `egui`
- Set the content type when uploading files to Google Cloud Storage and XBackbone.
- ***Breaking Change***
  * Change Type of `default_screenshot_type` in `UserConfig` to `LScreenshotType`

#### [0.7.7] - 2024-05-18
- Update localization files
- Fix \<br> being shown in message boxes
- Fix error messages not being displayed properly in message boxes
- Move logic for including localization files to it's own file.

#### [0.7.6] - 2024-05-17
- Update README
- Show message boxes as Alerts instead of Yes/No.
- Display Error Codes properly on app exit.
- Google Cloud Storage
    - Make `public_url_base` optional. Will default to `https://storage.googleapis.com/$bucket`.
    - Show notification when uploading.
- Fix clipboard not being set properly on Linux.

#### [0.7.5] - 2024-05-15
- Add support for Google Cloud Storage
- Fix some bugs with copying to clipboard

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