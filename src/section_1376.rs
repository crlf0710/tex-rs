//! @ The \.{\\language} extension is somewhat different.
//! We need a subroutine that comes into play when a character of
//! a non-|clang| language is being appended to the current paragraph.
//!
//! @<Declare action...@>=
//! procedure fix_language;
//! var @!l:ASCII_code; {the new current language}
//! begin if language<=0 then l:=0
//! else if language>255 then l:=0
//! else l:=language;
//! if l<>clang then
//!   begin new_whatsit(language_node,small_node_size);
//!   what_lang(tail):=l; clang:=l;@/
//!   what_lhm(tail):=norm_min(left_hyphen_min);
//!   what_rhm(tail):=norm_min(right_hyphen_min);
//!   end;
//! end;
//!
