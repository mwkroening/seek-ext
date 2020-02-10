use crate::{stream_len::StreamLen, stream_position::StreamPosition};
use futures::io::AsyncSeek;

pub trait AsyncSeekExt: AsyncSeek {
    fn stream_position(&mut self) -> StreamPosition<'_, Self>
    where
        Self: Unpin,
    {
        StreamPosition::new(self)
    }

    fn stream_len(&mut self) -> StreamLen<'_, Self>
    where
        Self: Unpin,
    {
        StreamLen::new(self)
    }
}

impl<S: AsyncSeek + ?Sized> AsyncSeekExt for S {}

#[cfg(test)]
mod tests {
    use super::AsyncSeekExt;
    use futures::{
        executor::block_on,
        io::{self, AsyncSeekExt as FuturesAsyncSeekExt, Cursor, SeekFrom},
    };

    async fn stream_len() -> io::Result<()> {
        let mut c = Cursor::new(vec![0; 15]);
        assert_eq!(c.stream_len().await?, 15);

        c.seek(SeekFrom::End(0)).await?;
        let old_pos = c.stream_position().await?;
        assert_eq!(c.stream_len().await?, 15);
        assert_eq!(c.stream_position().await?, old_pos);

        c.seek(SeekFrom::Start(7)).await?;
        c.seek(SeekFrom::Current(2)).await?;
        let old_pos = c.stream_position().await?;
        assert_eq!(c.stream_len().await?, 15);
        assert_eq!(c.stream_position().await?, old_pos);
        Ok(())
    }

    #[test]
    fn stream_len_test() -> io::Result<()> {
        block_on(stream_len())
    }

    async fn stream_position() -> io::Result<()> {
        // All `asserts` are duplicated here to make sure the method does not
        // change anything about the seek state.
        let mut c = Cursor::new(vec![0; 15]);
        assert_eq!(c.stream_position().await?, 0);
        assert_eq!(c.stream_position().await?, 0);

        c.seek(SeekFrom::End(0)).await?;
        assert_eq!(c.stream_position().await?, 15);
        assert_eq!(c.stream_position().await?, 15);

        c.seek(SeekFrom::Start(7)).await?;
        c.seek(SeekFrom::Current(2)).await?;
        assert_eq!(c.stream_position().await?, 9);
        assert_eq!(c.stream_position().await?, 9);

        c.seek(SeekFrom::End(-3)).await?;
        c.seek(SeekFrom::Current(1)).await?;
        c.seek(SeekFrom::Current(-5)).await?;
        assert_eq!(c.stream_position().await?, 8);
        assert_eq!(c.stream_position().await?, 8);
        Ok(())
    }

    #[test]
    fn stream_position_test() -> io::Result<()> {
        block_on(stream_position())
    }
}
