//! @ We get to the |final_cleanup| routine when \.{\\end} or \.{\\dump} has
//! been scanned and |its_all_over|\kern-2pt.

// @<Last-minute...@>=
// procedure final_cleanup;
#[allow(unused_variables)]
pub(crate) fn final_cleanup(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit;
    // var c:small_number; {0 for \.{\\end}, 1 for \.{\\dump}}
    /// 0 for `\end`, 1 for `\dump`
    let c;
    // begin c:=cur_chr; if c<>1 then new_line_char:=-1;
    c = globals.cur_chr.get();
    if c != 1 {
        new_line_char!(globals) = -1;
    }
    // if job_name=0 then open_log_file;
    if globals.job_name == 0 {
        open_log_file(globals);
    }
    // while input_ptr>0 do
    while globals.input_ptr > 0 {
        // if state=token_list then end_token_list@+else end_file_reading;
        if state!(globals) == token_list {
            end_token_list(globals);
        } else {
            end_file_reading(globals);
        }
    }
    // while open_parens>0 do
    while globals.open_parens > 0 {
        // begin print(" )"); decr(open_parens);
        print(globals, strpool_str!(" )").get() as _);
        decr!(globals.open_parens);
        // end;
    }
    // if cur_level>level_one then
    if globals.cur_level > level_one {
        // begin print_nl("("); print_esc("end occurred ");
        print_nl(globals, strpool_str!("("));
        print_esc(globals, strpool_str!("end occurred "));
        // print("inside a group at level ");
        print(globals, strpool_str!("inside a group at level ").get() as _);
        // @:end_}{\.{(\\end occurred...)}@>
        // print_int(cur_level-level_one); print_char(")");
        print_int(globals, (globals.cur_level - level_one).into());
        print_char(
            make_globals_io_string_log_view!(globals),
            ASCII_code_literal!(b')'),
        );
        // end;
    }
    // while cond_ptr<>null do
    while globals.cond_ptr != null {
        // begin print_nl("("); print_esc("end occurred ");
        print_nl(globals, strpool_str!("("));
        print_esc(globals, strpool_str!("end occurred "));
        // print("when "); print_cmd_chr(if_test,cur_if);
        print(globals, strpool_str!("when ").get() as _);
        print_cmd_chr(
            globals,
            if_test,
            chr_code_type::new(globals.cur_if.get() as _),
        );
        // if if_line<>0 then
        if globals.if_line != 0 {
            // begin print(" on line "); print_int(if_line);
            print(globals, strpool_str!(" on line ").get() as _);
            print_int(globals, globals.if_line);
            // end;
        }
        // print(" was incomplete)");
        print(globals, strpool_str!(" was incomplete)").get() as _);
        // if_line:=if_line_field(cond_ptr);
        globals.if_line = if_line_field!(globals, globals.cond_ptr);
        // cur_if:=subtype(cond_ptr); temp_ptr:=cond_ptr;
        globals.cur_if = subtype!(globals, globals.cond_ptr).into();
        globals.temp_ptr = globals.cond_ptr;
        // cond_ptr:=link(cond_ptr); free_node(temp_ptr,if_node_size);
        globals.cond_ptr = link!(globals, globals.cond_ptr);
        free_node(globals, globals.temp_ptr, if_node_size as _);
        // end;
    }
    // if history<>spotless then
    if globals.history != history_kind::spotless {
        // if ((history=warning_issued)or(interaction<error_stop_mode)) then
        if globals.history == history_kind::warning_issued || globals.interaction < error_stop_mode
        {
            // if selector=term_and_log then
            if globals.selector == term_and_log {
                // begin selector:=term_only;
                globals.selector = term_only.into();
                // print_nl("(see the transcript file for additional information)");
                print_nl(
                    globals,
                    strpool_str!("(see the transcript file for additional information)"),
                );
                // @.see the transcript file...@>
                // selector:=term_and_log;
                globals.selector = term_and_log.into();
                // end;
            }
        }
    }
    // if c=1 then
    if c == 1 {
        // begin @!init for c:=top_mark_code to split_bot_mark_code do
        region_initex! {
            for c in mark_code_kind::top_mark_code.get()..=mark_code_kind::split_bot_mark_code.get() {
                // if cur_mark[c]<>null then delete_token_ref(cur_mark[c]);
                if globals.cur_mark[c] != null {
                    delete_token_ref(globals, globals.cur_mark[c]);
                }
            }
            // if last_glue<>max_halfword then delete_glue_ref(last_glue);
            if globals.last_glue != max_halfword {
                delete_glue_ref(globals, globals.last_glue);
            }
            // store_fmt_file; return;@+tini@/
            store_fmt_file(globals)?;
            return_nojump!();
        }
        // print_nl("(\dump is performed only by INITEX)"); return;
        print_nl(globals, strpool_str!("(\\dump is performed only by INITEX)"));
        return_nojump!();
        // @:dump_}{\.{\\dump...only by INITEX}@>
        // end;
    }
    // exit:end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0054::term_and_log;
use crate::section_0054::term_only;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0062::print_nl;
use crate::section_0063::print_esc;
use crate::section_0065::print_int;
use crate::section_0073::error_stop_mode;
use crate::section_0076::history_kind;
use crate::section_0081::TeXResult;
use crate::section_0110::max_halfword;
use crate::section_0115::null;
use crate::section_0130::free_node;
use crate::section_0200::delete_token_ref;
use crate::section_0201::delete_glue_ref;
use crate::section_0210::if_test;
use crate::section_0221::level_one;
use crate::section_0297::chr_code_type;
use crate::section_0298::print_cmd_chr;
use crate::section_0307::token_list;
use crate::section_0324::end_token_list;
use crate::section_0329::end_file_reading;
use crate::section_0382::mark_code_kind;
use crate::section_0489::if_node_size;
use crate::section_0534::open_log_file;
use crate::section_1302::store_fmt_file;
