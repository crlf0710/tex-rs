//! ` `
// @<Print a symbolic description of this feasible break@>=
#[cfg(feature = "statistics")]
pub(crate) macro Print_a_symbolic_description_of_this_feasible_break($globals:expr, $r:expr, $d:expr, $b:expr, $pi:expr, $artificial_demerits:expr) {{
    // begin if printed_node<>cur_p then
    if $globals.printed_node != $globals.cur_p {
        // @<Print the list between |printed_node| and |cur_p|,
        //   then set |printed_node:=cur_p|@>;
        Print_the_list_between_printed_node_and_cur_p__then_set_printed_node_to_cur_p!($globals);
    }
    // print_nl("@@");
    print_nl($globals, crate::strpool_str!("@"));
    // @.\AT!@>
    // if cur_p=null then print_esc("par")
    if $globals.cur_p == null {
        print_esc($globals, crate::strpool_str!("par"));
    }
    // else if type(cur_p)<>glue_node then
    else if r#type!($globals, $globals.cur_p) != glue_node {
        // begin if type(cur_p)=penalty_node then print_esc("penalty")
        if r#type!($globals, $globals.cur_p) == penalty_node {
            print_esc($globals, crate::strpool_str!("penalty"));
        }
        // else if type(cur_p)=disc_node then print_esc("discretionary")
        else if r#type!($globals, $globals.cur_p) == disc_node {
            print_esc($globals, crate::strpool_str!("discretionary"));
        }
        // else if type(cur_p)=kern_node then print_esc("kern")
        else if r#type!($globals, $globals.cur_p) == kern_node {
            print_esc($globals, crate::strpool_str!("kern"));
        }
        // else print_esc("math");
        else {
            print_esc($globals, crate::strpool_str!("math"));
        }
        // end;
    }
    // print(" via @@@@");
    print($globals, crate::strpool_str!(" via @@").get() as _);
    // if break_node(r)=null then print_char("0")
    if break_node!($globals, $r) == null {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'0'),
        );
    }
    // else print_int(serial(break_node(r)));
    else {
        print_int($globals, serial!($globals, break_node!($globals, $r)) as _);
    }
    // print(" b=");
    print($globals, crate::strpool_str!(" b=").get() as _);
    // if b>inf_bad then print_char("*")@+else print_int(b);
    if $b > inf_bad {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'*'),
        );
    } else {
        print_int($globals, $b as _);
    }
    // @.*\relax@>
    // print(" p="); print_int(pi); print(" d=");
    print($globals, crate::strpool_str!(" p=").get() as _);
    print_int($globals, $pi);
    print($globals, crate::strpool_str!(" d=").get() as _);
    // if artificial_demerits then print_char("*")@+else print_int(d);
    if $artificial_demerits {
        print_char(
            make_globals_io_string_log_view!($globals),
            ASCII_code_literal!(b'*'),
        );
    } else {
        print_int($globals, $d);
    }
    // end
    use crate::section_0004::TeXGlobalsIoStringLogView;
    use crate::section_0058::print_char;
    use crate::section_0059::print;
    use crate::section_0062::print_nl;
    use crate::section_0063::print_esc;
    use crate::section_0065::print_int;
    use crate::section_0145::disc_node;
    use crate::section_0149::glue_node;
    use crate::section_0155::kern_node;
    use crate::section_0157::penalty_node;
}}
