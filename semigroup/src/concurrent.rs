use std::future::Future;

use futures::{Stream, StreamExt, TryStream, TryStreamExt};

use crate::{Commutative, Semigroup};

/// Async version of [`Semigroup`].
pub trait AsyncSemigroup: Semigroup {
    fn async_op_assign(base: &mut Self, other: Self) -> impl Future<Output = ()> {
        async { Semigroup::op_assign(base, other) }
    }
    fn async_op(mut base: Self, other: Self) -> impl Future<Output = Self> {
        async {
            Self::async_op_assign(&mut base, other).await;
            base
        }
    }
    fn async_semigroup_assign(&mut self, other: Self) -> impl Future<Output = ()> {
        async { Self::async_op_assign(self, other).await }
    }
    fn async_semigroup(self, other: Self) -> impl Future<Output = Self> {
        async { Self::async_op(self, other).await }
    }
}
impl<T: Semigroup> AsyncSemigroup for T {}

/// Async version of [`Commutative`].
pub trait AsyncCommutative: AsyncSemigroup + Commutative {
    /// Used by [`CombineStream::fold_semigroup`].
    fn fold_stream(init: Self, stream: impl Stream<Item = Self>) -> impl Future<Output = Self> {
        async { stream.fold(init, Self::async_op).await }
    }
    /// Used by [`CombineStream::reduce_semigroup`].
    fn reduce_stream(
        mut stream: impl Stream<Item = Self> + Unpin,
    ) -> impl Future<Output = Option<Self>> {
        async {
            let init = stream.next().await?;
            Some(Self::fold_stream(init, stream).await)
        }
    }
    /// Used by [`CombineStream::combine_monoid`].
    #[cfg(feature = "monoid")]
    fn combine_stream(stream: impl Stream<Item = Self>) -> impl Future<Output = Self>
    where
        Self: crate::Monoid,
    {
        async { Self::fold_stream(Self::identity(), stream).await }
    }
}
impl<T: Commutative> AsyncCommutative for T {}

/// Extensions for [`Stream`]s that items implement [`AsyncCommutative`]. Like [`crate::CombineIterator`].
pub trait CombineStream: Sized + Stream {
    /// This method like [`crate::CombineIterator::fold_final`], but stream.
    ///
    /// # Examples
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Sum, CombineStream, Semigroup};
    /// let s1 = futures::stream::iter(0..10);
    /// let sum = s1.map(Sum).fold_semigroup(Sum(0));
    /// assert_eq!(sum.await, Sum(45));
    ///
    /// let s2 = futures::stream::iter(0..0);
    /// let empty = s2.map(Sum).fold_semigroup(Sum(0));
    /// assert_eq!(empty.await, Sum(0))
    /// # });
    /// ```
    ///
    /// # Type safety
    /// This method is only available when item implements [`Commutative`].
    /// ```compile_fail
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Coalesce, CombineStream, Semigroup};
    /// let stream = futures::stream::iter(0..10);
    /// let cannot_coalesce = stream.map(Some).map(Coalesce).fold_semigroup(Coalesce(None));
    /// # });
    /// ```
    fn fold_semigroup(self, init: Self::Item) -> impl Future<Output = Self::Item>
    where
        Self::Item: AsyncCommutative,
    {
        Self::Item::fold_stream(init, self)
    }

    /// This method like [`crate::CombineIterator::lreduce`], but stream.
    ///
    /// # Example
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Sum, CombineStream, Semigroup};
    /// let s1 = futures::stream::iter(0..10);
    /// let sum = s1.map(Sum).reduce_semigroup();
    /// assert_eq!(sum.await, Some(Sum(45)));
    ///
    /// let s2 = futures::stream::iter(0..0);
    /// let empty = s2.map(Sum).reduce_semigroup();
    /// assert_eq!(empty.await, None)
    /// # });
    /// ```
    ///
    /// # Type safety
    /// This method is only available when item implements [`Commutative`].
    /// ```compile_fail
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Coalesce, CombineStream, Semigroup};
    /// let stream = futures::stream::iter(0..10);
    /// let cannot_coalesce = stream.map(Some).map(Coalesce).reduce_semigroup();
    /// # });
    /// ```
    fn reduce_semigroup(self) -> impl Future<Output = Option<Self::Item>>
    where
        Self: Unpin,
        Self::Item: AsyncCommutative,
    {
        Self::Item::reduce_stream(self)
    }

    /// This method like [`crate::CombineIterator::combine`], but stream.
    ///
    /// # Example
    /// ```
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Sum, CombineStream, Semigroup};
    /// let s1 = futures::stream::iter(0..10);
    /// let sum = s1.map(Sum).combine_monoid();
    /// assert_eq!(sum.await, Sum(45));
    ///
    /// let s2 = futures::stream::iter(0..0);
    /// let empty = s2.map(Sum).combine_monoid();
    /// assert_eq!(empty.await, Sum(0))
    /// # });
    /// ```
    ///
    /// # Type safety
    /// This method is only available when item implements [`Commutative`].
    /// ```compile_fail
    /// # futures::executor::block_on(async {
    /// use futures::StreamExt;
    /// use semigroup::{op::Coalesce, CombineStream, Semigroup};
    /// let stream = futures::stream::iter(0..10);
    /// let cannot_coalesce = stream.map(Some).map(Coalesce).combine_monoid();
    /// # });
    /// ```
    #[cfg(feature = "monoid")]
    fn combine_monoid(self) -> impl Future<Output = Self::Item>
    where
        Self::Item: AsyncCommutative + crate::Monoid,
    {
        Self::Item::combine_stream(self)
    }
}
impl<T: Stream> CombineStream for T {}

/// Async try version of [`Commutative`].
pub trait TryAsyncCommutative: AsyncCommutative {
    /// Used by [`TryCombineStream::try_fold_semigroup`].
    fn try_fold_stream<S: TryStream<Ok = Self>>(
        init: Self,
        stream: S,
    ) -> impl Future<Output = Result<Self, S::Error>> {
        let ok_async_op = |b, o| async { Ok(Self::async_op(b, o).await) };
        async move { stream.try_fold(init, ok_async_op).await }
    }
    /// Used by [`TryCombineStream::try_reduce_semigroup`].
    fn try_reduce_stream<S: TryStream<Item = Result<Self, E>, Ok = Self, Error = E> + Unpin, E>(
        mut stream: S,
    ) -> impl Future<Output = Option<Result<Self, E>>> {
        async {
            let init = stream.next().await?;
            match init {
                Ok(init) => Some(Self::try_fold_stream(init, stream).await),
                Err(err) => Some(Err(err)),
            }
        }
    }
    /// Used by [`TryCombineStream::try_combine_monoid`].
    #[cfg(feature = "monoid")]
    fn try_combine_stream<S: TryStream<Ok = Self>>(
        stream: S,
    ) -> impl Future<Output = Result<Self, S::Error>>
    where
        Self: crate::Monoid,
    {
        async { Self::try_fold_stream(Self::identity(), stream).await }
    }
}
impl<T: Commutative> TryAsyncCommutative for T {}

/// Extensions for [`TryStream`]s that items implement [`TryAsyncCommutative`]. Like [`crate::CombineIterator`].
pub trait TryCombineStream: Sized + TryStream {
    /// This method like [`crate::CombineIterator::fold_final`], but stream.
    ///
    /// # Examples
    /// ```
    /// # futures::executor::block_on(async {
    /// use std::convert::Infallible;
    /// use futures::StreamExt;
    /// use semigroup::{op::Sum, TryCombineStream, Semigroup};
    /// let s1 = futures::stream::iter((0..10).map(Sum).map(Ok::<_, Infallible>));
    /// let sum = s1.try_fold_semigroup(Sum(0));
    /// assert_eq!(sum.await, Ok(Sum(45)));
    ///
    /// let s2 = futures::stream::iter(vec![Ok(Sum(1)), Err(2), Ok(Sum(3))]);
    /// let empty = s2.try_fold_semigroup(Sum(0));
    /// assert_eq!(empty.await, Err(2));
    ///
    /// let s3 = futures::stream::iter((0..0).map(Sum).map(Ok::<_, Infallible>));
    /// let empty = s3.try_fold_semigroup(Sum(0));
    /// assert_eq!(empty.await, Ok(Sum(0)))
    /// # });
    /// ```
    ///
    /// # Type safety
    /// This method is only available when item implements [`Commutative`].
    /// Same to [`CombineStream::fold_semigroup`].
    fn try_fold_semigroup(
        self,
        init: Self::Ok,
    ) -> impl Future<Output = Result<Self::Ok, Self::Error>>
    where
        Self::Ok: TryAsyncCommutative,
    {
        Self::Ok::try_fold_stream(init, self)
    }

    /// This method like [`crate::CombineIterator::lreduce`], but stream.
    ///
    /// # Example
    /// ```
    /// # futures::executor::block_on(async {
    /// use std::convert::Infallible;
    /// use futures::StreamExt;
    /// use semigroup::{op::Sum, TryCombineStream, Semigroup};
    /// let s1 = futures::stream::iter(0..10).map(Sum).map(Ok::<_, Infallible>);
    /// let sum = s1.try_reduce_semigroup();
    /// assert_eq!(sum.await, Some(Ok(Sum(45))));
    ///
    /// let s2 = futures::stream::iter(vec![Ok(Sum(1)), Err(2), Ok(Sum(3))]);
    /// let empty = s2.try_reduce_semigroup();
    /// assert_eq!(empty.await, Some(Err(2)));
    ///
    /// let s3 = futures::stream::iter((0..0).map(Sum).map(Ok::<_, Infallible>));
    /// let empty = s3.try_reduce_semigroup();
    /// assert_eq!(empty.await, None);
    /// # });
    /// ```
    ///
    /// # Type safety
    /// This method is only available when item implements [`Commutative`].
    /// Same to [`CombineStream::reduce_semigroup`].
    /// ```
    fn try_reduce_semigroup<T, E>(
        self,
    ) -> impl Future<Output = Option<Result<Self::Ok, Self::Error>>>
    where
        T: TryAsyncCommutative,
        Self: Unpin,
        Self: TryStream<Item = Result<T, E>, Ok = T, Error = E>,
    {
        Self::Ok::try_reduce_stream(self)
    }
    /// This method like [`crate::CombineIterator::combine`], but stream.
    ///
    /// # Example
    /// ```
    /// # futures::executor::block_on(async {
    /// use std::convert::Infallible;
    /// use futures::StreamExt;
    /// use semigroup::{op::Sum, TryCombineStream, Semigroup};
    /// let s1 = futures::stream::iter(0..10).map(Sum).map(Ok::<_, Infallible>);
    /// let sum = s1.try_combine_monoid();
    /// assert_eq!(sum.await, Ok(Sum(45)));
    ///
    /// let s2 = futures::stream::iter(vec![Ok(Sum(1)), Err(2), Ok(Sum(3))]);
    /// let empty = s2.try_combine_monoid();
    /// assert_eq!(empty.await, Err(2));
    ///
    /// let s3 = futures::stream::iter((0..0).map(Sum).map(Ok::<_, Infallible>));
    /// let empty = s3.try_combine_monoid();
    /// assert_eq!(empty.await, Ok(Sum(0)))
    /// # });
    /// ```
    ///
    /// # Type safety
    /// This method is only available when item implements [`Commutative`].
    /// Same to [`CombineStream::combine_monoid`].
    #[cfg(feature = "monoid")]
    fn try_combine_monoid(self) -> impl Future<Output = Result<Self::Ok, Self::Error>>
    where
        Self::Ok: TryAsyncCommutative + crate::Monoid,
    {
        Self::Ok::try_combine_stream(self)
    }
}
impl<T: TryStream> TryCombineStream for T {}
