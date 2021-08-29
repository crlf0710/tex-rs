//! @ In a construction like `\.{\\if\\iftrue abc\\else d\\fi}', the first
//! \.{\\else} that we come to after learning that the \.{\\if} is false is
//! not the \.{\\else} we're looking for. Hence the following curious
//! logic is needed.
//!
