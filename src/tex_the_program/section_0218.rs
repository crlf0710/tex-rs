//! @ Here is a procedure that displays what \TeX\ is working on, at all levels.
//
// @p procedure@?print_totals; forward;@t\2@>
// procedure show_activities;
pub(crate) fn show_activities(globals: &mut TeXGlobals) {
    // var p:0..nest_size; {index into |nest|}
    // @!m:-mmode..mmode; {mode}
    // @!a:memory_word; {auxiliary}
    // @!q,@!r:pointer; {for showing the current page}
    // @!t:integer; {ditto}
    // begin nest[nest_ptr]:=cur_list; {put the top level into the array}
    /// put the top level into the array
    const _: () = ();
    globals.nest[globals.nest_ptr] = globals.cur_list;
    // print_nl(""); print_ln;
    print_nl(globals, crate::strpool_str!(""));
    print_ln(make_globals_io_string_log_view!(globals));
    // for p:=nest_ptr downto 0 do
    for p in (0..=globals.nest_ptr.get()).rev() {
        /// mode
        let m;
        /// auxiliary
        let a;
        // begin m:=nest[p].mode_field; a:=nest[p].aux_field;
        m = globals.nest[p].mode_field;
        a = globals.nest[p].aux_field;
        // print_nl("### "); print_mode(m);
        print_nl(globals, crate::strpool_str!("### "));
        print_mode(globals, m.get() as _);
        // print(" entered at line "); print_int(abs(nest[p].ml_field));
        print(globals, crate::strpool_str!(" entered at line ").get() as _);
        print_int(globals, globals.nest[p].ml_field.abs());
        // if m=hmode then if nest[p].pg_field <> @'40600000 then
        if m == hmode && globals.nest[p].pg_field != 0o40600000 {
            // begin print(" (language"); print_int(nest[p].pg_field mod @'200000);
            print(globals, crate::strpool_str!(" (language").get() as _);
            print_int(globals, globals.nest[p].pg_field % 0o200000);
            // print(":hyphenmin"); print_int(nest[p].pg_field div @'20000000);
            print(globals, crate::strpool_str!(":hyphenmin").get() as _);
            print_int(globals, globals.nest[p].pg_field / 0o20000000);
            // print_char(","); print_int((nest[p].pg_field div @'200000) mod @'100);
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code_literal!(b','),
            );
            print_int(globals, (globals.nest[p].pg_field / 200000) % 0o100);
            // print_char(")");
            print_char(
                make_globals_io_string_log_view!(globals),
                ASCII_code_literal!(b')'),
            );
            // end;
        }
        // if nest[p].ml_field<0 then print(" (\output routine)");
        if globals.nest[p].ml_field < 0 {
            print(
                globals,
                crate::strpool_str!(" (\\output routine)").get() as _,
            );
        }
        // if p=0 then
        if p == 0 {
            // begin @<Show the status of the current page@>;
            crate::section_0986::Show_the_status_of_the_current_page!(globals);
            // if link(contrib_head)<>null then
            if link!(globals, contrib_head) != null {
                // print_nl("### recent contributions:");
            }
            // end;
        }
        // show_box(link(nest[p].head_field));
        show_box(globals, link!(globals, globals.nest[p].head_field));
        // @<Show the auxiliary field, |a|@>;
        crate::section_0219::Show_the_auxiliary_field__a!(globals, m, a, p);
        // end;
    }
    // end;
}

use crate::section_0004::make_globals_io_string_log_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0018::ASCII_code_literal;
use crate::section_0057::print_ln;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0062::print_nl;
use crate::section_0065::print_int;
use crate::section_0115::null;
use crate::section_0118::link;
use crate::section_0162::contrib_head;
use crate::section_0198::show_box;
use crate::section_0211::hmode;
use crate::section_0211::print_mode;
