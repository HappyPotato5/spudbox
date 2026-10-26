
/*  harper:ignore
    Spudbox. A library that implements lots of useful data types.
    Copyright (C) 2026 HappyPotato5 (on github)

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

//! This submodule adds parsing utilities like the [`Parse`] and [`Kinded`] traits, and the types [`ParserStream`] and [`ParseError`].

use std::marker::PhantomData;

/// An error found during parsing.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ParseError;

/// A trait representing an object that a [`ParserStream`] can parse.
pub trait Parse<Token, Err: Default = ParseError>
where
    Self: Sized,
{
    fn parse(stream: ParserStream<Token, Err>) -> Result<Self, Err>;
}

/// A trait implemented by `Token` types that have an associated **kind**.
/// 
/// The *kind* of a `Token` represents **all** the possible *token types*, typically a *field-less* enum (*only unit variants*) fits this purpose.
pub trait Kinded {
    type Kind: Eq;

    fn kind(&self) -> Self::Kind;
}


/// A buffer used to parse *elements* from its *tokens*.
/// 
/// You can **choose** the type returned on **error** by setting the `Err` generic.
/// 
/// The returned `Err` in its [default](Default) form.
#[derive(Clone)]
pub struct ParserBuffer<'a, Token, Err: Default = ParseError> {
    cell: std::cell::Cell<Cursor<'a, Token>>,
    phantom: PhantomData<Err>
}

/// A stream used to *parse* elements from *tokens*.
/// 
/// You can **choose** the type returned on **error** by setting the `Err` generic.
/// 
/// The returned `Err` in its [default](Default) form.
/// 
/// # Examples
/// You can find examples in this crate's [GitHub repository](https://github.com/HappyPotato5/spudbox), in the *examples folder*.
pub type ParserStream<'a, Token, Err = ParseError> = &'a ParserBuffer<'a, Token, Err>;

impl<'a, Token, Err: Default> ParserBuffer<'a, Token, Err> {
    /// Constructs a new [`ParserBuffer`] from a sequence of Tokens.
    pub fn new(tokens: &'a [Token]) -> ParserBuffer<'a, Token, Err> {
        ParserBuffer {
            cell: std::cell::Cell::new(Cursor::new(tokens)),
            phantom: PhantomData
        }
    }

    /// Returns the position of the **first** unconsumed `Token` of the [`ParserStream`].
    pub fn pos(&self) -> usize {
        self.cell.get().pos
    }

    /// Peeks at the `ith` token, without consuming it.
    /// 
    /// [`peek(0)`](Self::peek()) returns the current token.
    /// 
    /// Returns the `ith` unconsumed `Token`.
    pub fn peek(&self, i: usize) -> Option<&'a Token> {
        self.cell
            .get()
            .peek(i)
    }

    /// If the current token equals the supplied `tok`, then it consumes and returns it.
    pub fn expect_token(&self, tok: &Token) -> Result<&'a Token, Err> 
        where Token: PartialEq 
    {
        let (res, next) = self.cell.get().step().ok_or(Err::default())?;
        if *res != *tok {
            return Err(Err::default());
        }
        self.cell.set(next);

        Ok(res)
    }

    /// If the current token's *kind* equals the supplied `kind`, then it consumes and returns it.
    pub fn expect_token_kind(&self, kind: &Token::Kind) -> Result<&'a Token, Err> 
        where Token: Kinded
    {
        let (res, next) = self.cell.get().step().ok_or(Err::default())?;
        if res.kind() != *kind {
            return Err(Err::default());
        }
        self.cell.set(next);

        Ok(res)
    }

    /// If the current token matches the `predicate`, it consumes and returns it.
    pub fn expect<P>(&self, predicate: P) -> Result<&'a Token, Err> 
        where P: FnOnce(&Token) -> bool 
    {
        let (res, next) = self.cell.get().step().ok_or(Err::default())?;
        if !predicate(res) {
            return Err(Err::default());
        }
        self.cell.set(next);

        Ok(res)
    }

    /// If the current token matches the `predicate`, returns the predicate's result, and consumes the token.
    /// 
    /// The predicate returns an [`Option`], [`Some(value)`](Some) => true. [`None`] => false.
    pub fn expect_and_map<F, R>(&self, predicate: F) -> Result<R, Err> 
        where F: FnOnce(&Token) -> Option<R> {
        let (res, next) = self.cell.get().step().ok_or(Err::default())?;
        if let Some(v) = predicate(res) {
            self.cell.set(next);
            Ok(v)
        } else {
            Err(Err::default())
        } 
    }

    /// Consumes **as many as possible** matching tokens, until it finds a *non-matching* token or *runs out of tokens*.
    /// 
    /// Returns an *empty* [`Vec`] if it didn't find any matching tokens.
    pub fn expect_many<F>(&self, mut predicate: F) -> Vec<&'a Token> 
        where F: FnMut(&Token) -> bool
    {
        let mut result = Vec::new();
        let mut pos = self.cell.get();

        while let Some((tok, next)) = pos.step() 
            {
            if !predicate(tok) { break; }
            result.push(tok);
            pos = next;
        }
        self.cell.set(pos);

        result
    }

    /// Consumes **as many as possible** tokens of the supplied `kind`, until it finds a token of a different kind or *runs out of tokens*.
    /// 
    /// Returns an *empty* [`Vec`] if it didn't find any token of the given `kind`.
    pub fn expect_many_of_kind(&self, kind: &Token::Kind) -> Vec<&'a Token> 
        where Token: Kinded
    {
        let mut result = Vec::new();
        let mut pos = self.cell.get();

        while let Some((tok, next)) = pos.step() 
            {
            if tok.kind() != *kind { break; }
            result.push(tok);
            pos = next;
        }
        self.cell.set(pos);

        result
    }

    /// Consumes **as many as possible** *matching* tokens, until *running out of tokens* or the `predicate` evaluates to [`None`].
    /// 
    /// The resulting [`Vec`] collects the unwrapped results of the `predicate` in the same order as the tokens found.
    /// 
    /// Returns an *empty* [`Vec`] if it didn't find any *matching* tokens.
    pub fn expect_and_map_many<F, R>(&self, mut predicate: F) -> Vec<R> 
        where F: FnMut(&Token) -> Option<R> {
        let mut result = Vec::new();
        let mut pos = self.cell.get();

        while let Some((tok, next)) = pos.step() {
            if let Some(v) = predicate(tok) {
                result.push(v);
                pos = next;
            } else { break; }
        }
        self.cell.set(pos);

        result
    }

    /// Tries to parse a `T` object with the remaining tokens.
    pub fn parse<T: Parse<Token, Err>>(&'a self) -> Result<T, Err> {
        let checkpoint = self.cell.get();
        match T::parse(self) {
            Ok(v) => Ok(v),
            Err(err) => {
                self.cell.set(checkpoint);
                Err(err)
            }
        }
    }

    /// Parses as many `Ts` as possible, until failure, or until it runs out of tokens.
    /// 
    /// Returns an *empty* [`Vec`] if it didn't find any *matching* tokens.
    pub fn parse_many<T: Parse<Token, Err>>(&'a self) -> Vec<T> {
        let mut result = Vec::new();

        while let Ok(t) = self.parse::<T>() {
            result.push(t);
        }
        result
    }

    /// Consumes and ignores as many as possible matching tokens, until finding a non-matching token, or running out of tokens.
    pub fn ignore_many<F>(&self, mut predicate: F) 
        where F: FnMut(&Token) -> bool
    {
        while let Some((tok, next)) = self.cell.get().step()
            && predicate(tok) {
            self.cell.set(next);
        }
    }

    /// Consumes the next token, if any, and returns it. 
    pub fn next(&self) -> Option<&'a Token> {
        let (tok, next) = self.cell.get().step()?;
        self.cell.set(next);
        Some(tok)
    }
}


/// A position in a token stream.
struct Cursor<'a, Token> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a, Token> Cursor<'a, Token> {
    /// Creates a new [`Cursor`] at position 0.
    fn new(tokens: &'a [Token]) -> Cursor<'a, Token> {
        Cursor {
            tokens: tokens,
            pos: 0,
        }
    }

    /// Peeks at the `ith` token starting at the current position.
    /// 
    /// [`peek(0)`](Self::peek()) will return the current token.
    fn peek(&self, i: usize) -> Option<&'a Token> {
        self.tokens.get(i)
    }

    /// Returns the current token and a cursor to the next one.
    fn step(&self) -> Option<(&'a Token, Cursor<'a, Token>)> {
        self.tokens.get(0).map(|x| (x, self.advanced()))
    }

    /// Returns a [`Cursor`] to the next token.
    fn advanced(&self) -> Cursor<'a, Token> {
        let mut res = self.clone();
        res.tokens = &res.tokens[1..];
        res.pos += 1;
        res
    }
}

impl<'a, Token> Clone for Cursor<'a, Token> {
    fn clone(&self) -> Self {
        Self { tokens: self.tokens, pos: self.pos.clone() }
    }
}

impl<'a, Token> Copy for Cursor<'a, Token> {}
