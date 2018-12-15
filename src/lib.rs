use std::ops::Range;

pub mod support;

/// A generic lexer trait
///
/// This trait enables lexers to be used by any parsers supporting `plug_::Lexer`.
///
/// ## Advantages
///
/// The advantage for users is the ability to use any lexer or tokenizer that implements `Lexer`
/// without any additional work.
/// The advantage for parser authors is that they can use a single, generic Parser implementation
/// that relies on `Lexer` without having to restrict it to a specific tokenizer.
/// The advantage for lexer authors is that more users can use their lexer libraries more easily.
pub trait Lexer<'source, Token, InputSlice>
where
    InputSlice: 'source,
{
    const ERROR: Token;
    const END: Token;

    fn advance(&mut self);
    fn terminal<'t, 'lexer: 't>(&'lexer self) -> &'t Token;
    fn range(&self) -> Range<usize>;
    fn slice(&self) -> InputSlice;
}
