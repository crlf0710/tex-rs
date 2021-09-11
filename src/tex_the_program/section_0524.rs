//! @ Here is the only place we use |pack_buffered_name|. This part of the program
//! becomes active when a ``virgin'' \TeX\ is trying to get going, just after
//! the preliminary initialization, or when the user is substituting another
//! format file by typing `\.\&' after the initial `\.{**}' prompt.  The buffer
//! contains the first line of input in |buffer[loc..(last-1)]|, where
//! |loc<last| and |buffer[loc]<>" "|.
//
// @<Declare the function called |open_fmt_file|@>=
// function open_fmt_file:boolean;
pub(crate) fn open_fmt_file(globals: &mut TeXGlobals) -> boolean {
    // label found,exit;
    // var j:0..buf_size; {the first space after the format file name}
    /// the first space after the format file name
    let mut j;
    // begin j:=loc;
    j = loc!(globals);
    crate::region_forward_label!(
    |'found|
    {
    // if buffer[loc]="&" then
    if globals.buffer[loc!(globals)] == ASCII_code_literal!(b'&') {
        // begin incr(loc); j:=loc; buffer[last]:=" ";
        incr!(loc!(globals));
        j = loc!(globals);
        globals.buffer[globals.last] = ASCII_code_literal!(b' ');
        // while buffer[j]<>" " do incr(j);
        while globals.buffer[j] != ASCII_code_literal!(b' ') {
            incr!(j);
        }
        // pack_buffered_name(0,loc,j-1); {try first without the system file area}
        /// try first without the system file area
        pack_buffered_name(globals, 0.into(), loc!(globals) as integer, j as integer - 1);
        // if w_open_in(fmt_file) then goto found;
        if w_open_in(make_globals_filename_view!(globals), &mut globals.fmt_file) {
            crate::goto_forward_label!('found);
        }
        // pack_buffered_name(format_area_length,loc,j-1);
        //   {now try the system format file area}
        /// now try the system format file area
        pack_buffered_name(globals, small_number::new(format_area_length as _), loc!(globals) as integer, j as integer - 1);
        // if w_open_in(fmt_file) then goto found;
        if w_open_in(make_globals_filename_view!(globals), &mut globals.fmt_file) {
            crate::goto_forward_label!('found);
        }
        // wake_up_terminal;
        wake_up_terminal(globals);
        // wterm_ln('Sorry, I can''t find that format;',' will try PLAIN.');
        wterm_ln(make_globals_io_view!(globals), "Sorry, I can't find that format; will try PLAIN.");
        // @.Sorry, I can't find...@>
        // update_terminal;
        update_terminal(globals);
        // end;
        //   {now pull out all the stops: try for the system \.{plain} file}
        /// now pull out all the stops: try for the system `plain` file
        const _ : () = ();
    }
    // pack_buffered_name(format_default_length-format_ext_length,1,0);
    pack_buffered_name(globals, small_number::new((format_default_length - format_ext_length) as _), 1, 0);
    // if not w_open_in(fmt_file) then
    if !w_open_in(make_globals_filename_view!(globals), &mut globals.fmt_file) {
        // begin wake_up_terminal;
        wake_up_terminal(globals);
        // wterm_ln('I can''t find the PLAIN format file!');
        wterm_ln(make_globals_io_view!(globals), "I can't find the PLAIN format file!");
        // @.I can't find PLAIN...@>
        // @.plain@>
        // open_fmt_file:=false; return;
        return false;
        // end;
    }
    }
    // found:loc:=j; open_fmt_file:=true;
    'found <-
    );
    loc!(globals) = j;
    true
    // exit:end;
}

use crate::pascal::boolean;
use crate::pascal::integer;
use crate::section_0004::make_globals_filename_view;
use crate::section_0004::make_globals_io_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsFilenameView;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0016::incr;
use crate::section_0018::ASCII_code_literal;
use crate::section_0027::w_open_in;
use crate::section_0034::update_terminal;
use crate::section_0034::wake_up_terminal;
use crate::section_0036::loc;
use crate::section_0056::wterm_ln;
use crate::section_0101::small_number;
use crate::section_0520::format_area_length;
use crate::section_0520::format_default_length;
use crate::section_0520::format_ext_length;
use crate::section_0523::pack_buffered_name;
