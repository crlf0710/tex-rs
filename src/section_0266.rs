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
        // after_assignment: print_esc("afterassignment");
        // after_group: print_esc("aftergroup");
        // assign_font_dimen: print_esc("fontdimen");
        // begin_group: print_esc("begingroup");
        // break_penalty: print_esc("penalty");
        else if $cmd == break_penalty {
            print_esc($globals, strpool_str!("penalty"));
            true
        }
        // char_num: print_esc("char");
        // cs_name: print_esc("csname");
        // def_font: print_esc("font");
        // delim_num: print_esc("delimiter");
        // divide: print_esc("divide");
        // end_cs_name: print_esc("endcsname");
        // end_group: print_esc("endgroup");
        // ex_space: print_esc(" ");
        else if $cmd == ex_space {
            print_esc($globals, strpool_str!(" "));
            true
        }
        // expand_after: print_esc("expandafter");
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
        // no_align: print_esc("noalign");
        // no_boundary:print_esc("noboundary");
        // no_expand: print_esc("noexpand");
        // non_script: print_esc("nonscript");
        // omit: print_esc("omit");
        // radical: print_esc("radical");
        // read_to_cs: print_esc("read");
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
