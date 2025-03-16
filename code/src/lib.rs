//#![deny(missing_docs)]
//#![doc = include_str!("../readme.md")]

pub mod averages;
pub mod bytes;
pub mod color;
pub mod command;
pub mod experience;
pub mod fimfiction_api;
pub mod fs;
pub mod http;
pub mod json;
pub mod markdown;
pub mod number_format;
pub mod pagination;
pub mod passphrase;
pub mod regex;
pub mod sprite_sheet;
pub mod stderr;
pub mod stdin;
pub mod string_image;
pub mod threads;
pub mod time;
pub mod traits;
pub mod word_stats;

#[allow(clippy::tabs_in_doc_comments)]
/// Fix for glibc memory bug.
///
/// glibc malloc has a long standing bug
/// causing memory hoarding.
///
/// This module fixes that.
/// Code originally written by [Michael Murphy](https://github.com/mmstick).
///
/// Re-licensed under MIT with permission.
///
/// To use it, simply add the following to your mane function:
/// ```rs
/// 	pony::malloc::limit_mmap_threshold(65_536);
/// ```
///
/// It is also suggested to call `trim(0)`
/// from time to time or in a loop.
/// ```rs
/// 	pony::malloc::trim(0);
/// ```
///
/// More details on this can be read from the following:
/// - <https://github.com/pop-os/libcosmic/blob/master/src/malloc.rs>
/// - <https://fosstodon.org/@mmstick/113952008189644564>
/// - <https://github.com/pop-os/cosmic-bg/pull/73>
/// - <https://www.man7.org/linux/man-pages/man3/mallopt.3.html>
/// - <https://www.man7.org/linux/man-pages/man3/malloc_trim.3.html>
#[cfg(target_env = "gnu")]
pub mod malloc;
