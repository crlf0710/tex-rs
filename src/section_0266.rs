//! @ Each primitive has a corresponding inverse, so that it is possible to
//! display the cryptic numeric contents of |eqtb| in symbolic form.
//! Every call of |primitive| in this program is therefore accompanied by some
//! straightforward code that forms part of the |print_cmd_chr| routine
//! below.
//
// @<Cases of |print_cmd_chr|...@>=
macro_rules! Cases_of_print_cmd_chr_for_symbolic_printing_of_primitives_0266 {
    ($globals:expr, $cmd:expr, $chr_code:expr) => {{
        if false {
            unreachable!();
        }
        // accent: print_esc("accent");
        // advance: print_esc("advance");
        else if $cmd == advance {
            print_esc($globals, strpool_str!("advance"));
            true
        }
        // after_assignment: print_esc("afterassignment");
        // after_group: print_esc("aftergroup");
        // assign_font_dimen: print_esc("fontdimen");
        else if $cmd == assign_font_dimen {
            print_esc($globals, strpool_str!("fontdimen"));
            true
        }
        // begin_group: print_esc("begingroup");
        else if $cmd == begin_group {
            print_esc($globals, strpool_str!("begingroup"));
            true
        }
        // break_penalty: print_esc("penalty");
        else if $cmd == break_penalty {
            print_esc($globals, strpool_str!("penalty"));
            true
        }
        // char_num: print_esc("char");
        // cs_name: print_esc("csname");
        else if $cmd == cs_name {
            print_esc($globals, strpool_str!("csname"));
            true
        }
        // def_font: print_esc("font");
        else if $cmd == def_font {
            print_esc($globals, strpool_str!("font"));
            true
        }
        // delim_num: print_esc("delimiter");
        // divide: print_esc("divide");
        else if $cmd == divide {
            print_esc($globals, strpool_str!("divide"));
            true
        }
        // end_cs_name: print_esc("endcsname");
        else if $cmd == end_cs_name {
            print_esc($globals, strpool_str!("endcsname"));
            true
        }
        // end_group: print_esc("endgroup");
        else if $cmd == end_group {
            print_esc($globals, strpool_str!("endgroup"));
            true
        }
        // ex_space: print_esc(" ");
        else if $cmd == ex_space {
            print_esc($globals, strpool_str!(" "));
            true
        }
        // expand_after: print_esc("expandafter");
        else if $cmd == expand_after {
            print_esc($globals, strpool_str!("expandafter"));
            true
        }
        // halign: print_esc("halign");
        // hrule: print_esc("hrule");
        // ignore_spaces: print_esc("ignorespaces");
        // insert: print_esc("insert");
        // ital_corr: print_esc("/");
        // mark: print_esc("mark");
        // math_accent: print_esc("mathaccent");
        // math_char_num: print_esc("mathchar");
        // math_choice: print_esc("mathchoice");
        // multiply: print_esc("multiply");
        else if $cmd == multiply {
            print_esc($globals, strpool_str!("multiply"));
            true
        }
        // no_align: print_esc("noalign");
        // no_boundary:print_esc("noboundary");
        // no_expand: print_esc("noexpand");
        else if $cmd == no_expand {
            print_esc($globals, strpool_str!("noexpand"));
            true
        }
        // non_script: print_esc("nonscript");
        // omit: print_esc("omit");
        // radical: print_esc("radical");
        // read_to_cs: print_esc("read");
        else if $cmd == read_to_cs {
            print_esc($globals, strpool_str!("read"));
            true
        }
        // relax: print_esc("relax");
        else if $cmd == relax {
            print_esc($globals, strpool_str!("relax"));
            true
        }
        // set_box: print_esc("setbox");
        else if $cmd == set_box {
            print_esc($globals, strpool_str!("setbox"));
            true
        }
        // set_prev_graf: print_esc("prevgraf");
        // set_shape: print_esc("parshape");
        // the: print_esc("the");
        else if $cmd == the {
            print_esc($globals, strpool_str!("the"));
            true
        }
        // toks_register: print_esc("toks");
        // vadjust: print_esc("vadjust");
        // valign: print_esc("valign");
        // vcenter: print_esc("vcenter");
        // vrule: print_esc("vrule");
        else if $cmd == vrule {
            print_esc($globals, strpool_str!("vrule"));
            true
        }
        else {
            false
        }
    }}
}
