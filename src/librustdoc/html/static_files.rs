// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Static files bundled with documentation output.
//!
//! All the static files are included here for centralized access in case anything other than the
//! HTML rendering code (say, the theme checker) needs to access one of these files.
//!
//! Note about types: CSS and JavaScript files are included as `&'static str` to allow for the
//! minifier to run on them. All other files are included as `&'static [u8]` so they can be
//! directly written to a `Write` handle.

/// The file contents of the main `rustdoc.css` file, responsible for the core layout of the page.
pub static RUSTDOC_CSS: &'static str = include_str!("static/rustdoc.css");

/// The file contents of `settings.css`, responsible for the items on the settings page.
pub static SETTINGS_CSS: &'static str = include_str!("static/settings.css");

/// The file contents of the `noscript.css` file, used in case JS isn't supported or is disabled.
pub static NOSCRIPT_CSS: &'static str = include_str!("static/noscript.css");

/// The file contents of `normalize.css`, included to even out standard elements between browser
/// implementations.
pub static NORMALIZE_CSS: &'static str = include_str!("static/normalize.css");

/// The file contents of `main.js`, which contains the core JavaScript used on documentation pages,
/// including search behavior and docblock folding, among others.
pub static MAIN_JS: &'static str = include_str!("static/main.js");

/// The file contents of `settings.js`, which contains the JavaScript used to handle the settings
/// page.
pub static SETTINGS_JS: &'static str = include_str!("static/settings.js");

/// The file contents of `storage.js`, which contains functionality related to browser Local
/// Storage, used to store documentation settings.
pub static STORAGE_JS: &'static str = include_str!("static/storage.js");

/// The file contents of `brush.svg`, the icon used for the theme-switch button.
pub static BRUSH_SVG: &'static [u8] = include_bytes!("static/brush.svg");

/// The file contents of `wheel.svg`, the icon used for the settings button.
pub static WHEEL_SVG: &'static [u8] = include_bytes!("static/wheel.svg");

/// The file contents of `down-arrow.svg`, the icon used for the crate choice combobox.
pub static DOWN_ARROW_SVG: &'static [u8] = include_bytes!("static/down-arrow.svg");

/// The contents of `COPYRIGHT.txt`, the license listing for files distributed with documentation
/// output.
pub static COPYRIGHT: &'static [u8] = include_bytes!("static/COPYRIGHT.txt");

/// The contents of `LICENSE-APACHE.txt`, the text of the Apache License, version 2.0.
pub static LICENSE_APACHE: &'static [u8] = include_bytes!("static/LICENSE-APACHE.txt");

/// The contents of `LICENSE-MIT.txt`, the text of the MIT License.
pub static LICENSE_MIT: &'static [u8] = include_bytes!("static/LICENSE-MIT.txt");

/// The built-in themes given to every documentation site.
pub mod themes {
    /// The "light" theme, selected by default when no setting is available. Used as the basis for
    /// the `--theme-checker` functionality.
    pub static LIGHT: &'static str = include_str!("static/themes/light.css");

    /// The "dark" theme.
    pub static DARK: &'static str = include_str!("static/themes/dark.css");
}

/// Files related to the Fira Sans font.
pub mod fira_sans {
    /// The file `FiraSans-Regular.woff`, the Regular variant of the Fira Sans font.
    pub static REGULAR: &'static [u8] = include_bytes!("static/FiraSans-Regular.woff");

    /// The file `FiraSans-Medium.woff`, the Medium variant of the Fira Sans font.
    pub static MEDIUM: &'static [u8] = include_bytes!("static/FiraSans-Medium.woff");

    /// The file `FiraSans-LICENSE.txt`, the license text for the Fira Sans font.
    pub static LICENSE: &'static [u8] = include_bytes!("static/FiraSans-LICENSE.txt");
}

/// Files related to the Heuristica font.
pub mod heuristica {
    /// The file `Heuristica-Italic.woff`, the Italic variant of the Heuristica font.
    pub static ITALIC: &'static [u8] = include_bytes!("static/Heuristica-Italic.woff");

    /// The file `Heuristica-LICENSE.txt`, the license text for the Heuristica font.
    pub static LICENSE: &'static [u8] = include_bytes!("static/Heuristica-LICENSE.txt");
}

/// Files related to the Source Serif Pro font.
pub mod source_serif_pro {
    /// The file `SourceSerifPro-Regular.woff`, the Regular variant of the Source Serif Pro font.
    pub static REGULAR: &'static [u8] = include_bytes!("static/SourceSerifPro-Regular.woff");

    /// The file `SourceSerifPro-Bold.woff`, the Bold variant of the Source Serif Pro font.
    pub static BOLD: &'static [u8] = include_bytes!("static/SourceSerifPro-Bold.woff");

    /// The file `SourceSerifPro-LICENSE.txt`, the license text for the Source Serif Pro font.
    pub static LICENSE: &'static [u8] = include_bytes!("static/SourceSerifPro-LICENSE.txt");
}

/// Files related to the Source Code Pro font.
pub mod source_code_pro {
    /// The file `SourceCodePro-Regular.woff`, the Regular variant of the Source Code Pro font.
    pub static REGULAR: &'static [u8] = include_bytes!("static/SourceCodePro-Regular.woff");

    /// The file `SourceCodePro-Semibold.woff`, the Semibold variant of the Source Code Pro font.
    pub static SEMIBOLD: &'static [u8] = include_bytes!("static/SourceCodePro-Semibold.woff");

    /// The file `SourceCodePro-LICENSE.txt`, the license text of the Source Code Pro font.
    pub static LICENSE: &'static [u8] = include_bytes!("static/SourceCodePro-LICENSE.txt");
}

/// Files related to the sidebar in rustdoc sources.
pub mod sidebar {
    /// File script to handle sidebar.
    pub static SOURCE_SCRIPT: &'static str = include_str!("static/source-script.js");
}
