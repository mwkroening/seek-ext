#[allow(unstable_name_collisions)]
#[cfg(feature="std")]
pub mod seek_ext;
#[cfg(feature="std")]
pub use crate::seek_ext::SeekExt;

#[cfg(feature="async")]
pub mod async_seek_ext;
#[cfg(feature="async")]
pub use async_seek_ext::AsyncSeekExt;
#[cfg(feature="async")]
mod stream_len;
#[cfg(feature="async")]
mod stream_position;
