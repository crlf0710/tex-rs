//! @ The sixteen possible \.{\\write} streams are represented by the |write_file|
//! array. The |j|th file is open if and only if |write_open[j]=true|. The last
//! two streams are special; |write_open[16]| represents a stream number
//! greater than 15, while |write_open[17]| represents a negative stream number,
//! and both of these variables are always |false|.
//!
//! @<Glob...@>=
//! @!write_file:array[0..15] of alpha_file;
//! @!write_open:array[0..17] of boolean;
//!
//! @ @<Set init...@>=
//! for k:=0 to 17 do write_open[k]:=false;
//!
