use crate::async_seek_ext::AsyncSeekExt;
use futures::{future::Future, io::AsyncSeek, ready};
use std::{
    io::{self, SeekFrom},
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct StreamLen<'a, S: ?Sized> {
    seek: &'a mut S,
}

impl<S: ?Sized + Unpin> Unpin for StreamLen<'_, S> {}

impl<'a, S: AsyncSeek + ?Sized + Unpin> StreamLen<'a, S> {
    pub(super) fn new(seek: &'a mut S) -> Self {
        Self { seek }
    }
}

impl<S: AsyncSeek + ?Sized + Unpin> Future for StreamLen<'_, S> {
    type Output = io::Result<u64>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = &mut *self;
        let old_pos = ready!(Pin::new(&mut this.seek.stream_position()).poll(cx)?);
        let len = ready!(Pin::new(&mut this.seek).poll_seek(cx, SeekFrom::End(0))?);

        if old_pos != len {
            ready!(Pin::new(&mut this.seek).poll_seek(cx, SeekFrom::Start(old_pos))?);
        }

        Poll::Ready(Ok(len))
    }
}
