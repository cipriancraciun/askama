//! **This is a patched fork of Askama!**
//!
//! ----
//!
//! Askama implements a type-safe compiler for Jinja-like templates.
//! It lets you write templates in a Jinja-like syntax,
//! which are linked to a `struct` defining the template context.
//! This is done using a custom derive implementation (implemented
//! in [`askama_derive`](https://crates.io/crates/askama_patched_derive)).
//!
//! For feature highlights and a quick start, please review the
//! [README](https://github.com/cipriancraciun/askama/blob/main/README.md).
//!
//! The primary documentation for this crate now lives in
//! [the book](https://djc.github.io/askama/).
//!
//! # Creating Askama templates
//!
//! An Askama template is a `struct` definition which provides the template
//! context combined with a UTF-8 encoded text file (or inline source, see
//! below). Askama can be used to generate any kind of text-based format.
//! The template file's extension may be used to provide content type hints.
//!
//! A template consists of **text contents**, which are passed through as-is,
//! **expressions**, which get replaced with content while being rendered, and
//! **tags**, which control the template's logic.
//! The template syntax is very similar to [Jinja](http://jinja.pocoo.org/),
//! as well as Jinja-derivatives like [Twig](http://twig.sensiolabs.org/) or
//! [Tera](https://github.com/Keats/tera).
//!
//! ## The `template()` attribute
//!
//! Askama works by generating one or more trait implementations for any
//! `struct` type decorated with the `#[derive(Template)]` attribute. The
//! code generation process takes some options that can be specified through
//! the `template()` attribute. The following sub-attributes are currently
//! recognized:
//!
//! * `path` (as `path = "foo.html"`): sets the path to the template file. The
//!   path is interpreted as relative to the configured template directories
//!   (by default, this is a `templates` directory next to your `Cargo.toml`).
//!   The file name extension is used to infer an escape mode (see below). In
//!   web framework integrations, the path's extension may also be used to
//!   infer the content type of the resulting response.
//!   Cannot be used together with `source`.
//! * `source` (as `source = "{{ foo }}"`): directly sets the template source.
//!   This can be useful for test cases or short templates. The generated path
//!   is undefined, which generally makes it impossible to refer to this
//!   template from other templates. If `source` is specified, `ext` must also
//!   be specified (see below). Cannot be used together with `path`.
//! * `ext` (as `ext = "txt"`): lets you specify the content type as a file
//!   extension. This is used to infer an escape mode (see below), and some
//!   web framework integrations use it to determine the content type.
//!   Cannot be used together with `path`.
//! * `print` (as `print = "code"`): enable debugging by printing nothing
//!   (`none`), the parsed syntax tree (`ast`), the generated code (`code`)
//!   or `all` for both. The requested data will be printed to stdout at
//!   compile time.
//! * `escape` (as `escape = "none"`): override the template's extension used for
//!   the purpose of determining the escaper for this template. See the section
//!   on configuring custom escapers for more information.
//! * `syntax` (as `syntax = "foo"`): set the syntax name for a parser defined
//!   in the configuration file. The default syntax , "default",  is the one
//!   provided by Askama.

#![allow(unused_imports)]
#[macro_use]
extern crate askama_patched_derive;
pub use askama_patched_shared as shared;

use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub use askama_patched_escape::{Html, Text};

/// Main `Template` trait; implementations are generally derived
pub trait Template {
    /// Helper method which allocates a new `String` and renders into it
    fn render(&self) -> Result<String> {
        let mut buf = String::with_capacity(self.size_hint());
        self.render_into(&mut buf)?;
        Ok(buf)
    }
    /// Renders the template to the given `writer` buffer
    fn render_into(&self, writer: &mut dyn std::fmt::Write) -> Result<()>;
    /// Helper method which allocates a new `Vec<u8>` and renders into it
    fn render_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(self.size_hint());
        self.render_into_bytes(&mut buf)?;
        Ok(buf)
    }
    /// Renders the template to the given `writer`
    ///
    /// It is recommended to use a buffered implementation.
    fn render_into_bytes(&self, writer: &mut dyn std::io::Write) -> Result<()> {
        let mut adapter = shared::io::WriteIoToFmt::new(writer);
        let result = self.render_into(&mut adapter);
        match (adapter.error(), result) {
            (Some(err), _) => Err(Error::Io(err)),
            (None, result) => result,
        }
    }
    /// Helper function to inspect the template's extension
    fn extension(&self) -> Option<&'static str>;
    /// Provides an conservative estimate of the expanded length of the rendered template
    fn size_hint(&self) -> usize;
}

pub trait SizedTemplate {
    /// Renders the template to the given `writer` buffer
    fn write_into<W: std::fmt::Write + ?Sized>(&self, writer: &mut W) -> Result<()>;
    /// Renders the template to the given `writer`. (It is recommended to use a buffered implementation.)
    fn write_into_bytes<W: std::io::Write + ?Sized>(&self, writer: &mut W) -> Result<()> {
        let mut adapter = shared::io::WriteIoToFmt::new(writer);
        let result = self.write_into(&mut adapter);
        match (adapter.error(), result) {
            (Some(err), _) => Err(Error::Io(err)),
            (None, result) => result,
        }
    }
    /// Helper function to inspect the template's extension
    fn extension() -> Option<&'static str>;
    /// Provides an conservative estimate of the expanded length of the rendered template
    fn size_hint() -> usize;
}

pub use crate::shared::filters;
pub use crate::shared::helpers;
pub use crate::shared::{read_config_file, Error, MarkupDisplay, Result};
pub use askama_patched_derive::*;

pub mod mime {
    #[cfg(all(feature = "mime_guess", feature = "mime"))]
    pub fn extension_to_mime_type(ext: &str) -> mime_guess::Mime {
        let basic_type = mime_guess::from_ext(ext).first_or_octet_stream();
        for (simple, utf_8) in &TEXT_TYPES {
            if &basic_type == simple {
                return utf_8.clone();
            }
        }
        basic_type
    }

    #[cfg(all(feature = "mime_guess", feature = "mime"))]
    const TEXT_TYPES: [(mime_guess::Mime, mime_guess::Mime); 6] = [
        (mime::TEXT_PLAIN, mime::TEXT_PLAIN_UTF_8),
        (mime::TEXT_HTML, mime::TEXT_HTML_UTF_8),
        (mime::TEXT_CSS, mime::TEXT_CSS_UTF_8),
        (mime::TEXT_CSV, mime::TEXT_CSV_UTF_8),
        (
            mime::TEXT_TAB_SEPARATED_VALUES,
            mime::TEXT_TAB_SEPARATED_VALUES_UTF_8,
        ),
        (
            mime::APPLICATION_JAVASCRIPT,
            mime::APPLICATION_JAVASCRIPT_UTF_8,
        ),
    ];
}

/// Old build script helper to rebuild crates if contained templates have changed
///
/// This function is now deprecated and does nothing.
#[deprecated(
    since = "0.8.1",
    note = "file-level dependency tracking is handled automatically without build script"
)]
pub fn rerun_if_templates_changed() {}
