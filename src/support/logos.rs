use std::ops::Range;
use std::marker::PhantomData;

pub struct LogosShim<'source, T, Input>
where
    T: logos::Logos,
    Input: logos::Source<'source>,
{
    inner: logos::Lexer<T, Input>,
    #[allow(dead_code)]
    phantom: &'source PhantomData<Input>,
}

impl<'a, T, I> LogosShim<'a, T, I>
where
    T: logos::Logos,
    I: logos::Source<'a>,
{
    pub fn wrap(inner: logos::Lexer<T, I>) -> Self {
        Self {
            inner,
            phantom: &PhantomData {},
        }
    }
}

impl<'source, T, Input, Slice> crate::Lexer<'source, T, Slice> for LogosShim<'source, T, Input>
where
    T: logos::Logos,
    Input: logos::Source<'source>,
    Slice: logos::Slice<'source> + 'source,
    Slice: std::convert::From<<Input as logos::Source<'source>>::Slice>,
{
    const ERROR: T = <T as logos::Logos>::ERROR;
    const END: T = <T as logos::Logos>::END;

    fn advance(&mut self) {
        self.inner.advance();
    }

    fn terminal<'t, 'lexer: 't>(&'lexer self) -> &'t T {
        &self.inner.token
    }

    fn range(&self) -> Range<usize> {
        self.inner.range()
    }

    fn slice(&self) -> Slice {
        self.inner.slice().into()
    }
}
