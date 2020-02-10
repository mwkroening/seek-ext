use futures::{future::Future, io::AsyncSeek};
use std::{
    io::{self, SeekFrom},
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct StreamPosition<'a, S: ?Sized> {
    seek: &'a mut S,
}

impl<S: ?Sized + Unpin> Unpin for StreamPosition<'_, S> {}

impl<'a, S: AsyncSeek + ?Sized + Unpin> StreamPosition<'a, S> {
    pub(super) fn new(seek: &'a mut S) -> Self {
        Self { seek }
    }
}

impl<S: AsyncSeek + ?Sized + Unpin> Future for StreamPosition<'_, S> {
    type Output = io::Result<u64>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        Pin::new(&mut this.seek).poll_seek(cx, SeekFrom::Current(0))
    }
}
