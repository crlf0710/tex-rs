//! @ We shall concentrate first on the inner loop of |main_control|, deferring
//! consideration of the other cases until later.
//! @^inner loop@>
//
// @d big_switch=60 {go here to branch on the next token of input}
// @d main_loop=70 {go here to typeset a string of consecutive characters}
// @d main_loop_wrapup=80 {go here to finish a character or ligature}
// @d main_loop_move=90 {go here to advance the ligature cursor}
// @d main_loop_move_lig=95 {same, when advancing past a generated ligature}
// @d main_loop_lookahead=100 {go here to bring in another character, if any}
// @d main_lig_loop=110 {go here to check for ligatures or kerning}
// @d append_normal_space=120 {go here to append a normal space between words}
//
// @p @t\4@>@<Declare action procedures for use by |main_control|@>@;
// @t\4@>@<Declare the procedure called |handle_right_brace|@>@;

// procedure main_control; {governs \TeX's activities}

/// governs `TeX`'s activities
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn main_control(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label big_switch,reswitch,main_loop,main_loop_wrapup,
    //   main_loop_move,main_loop_move+1,main_loop_move+2,main_loop_move_lig,
    //   main_loop_lookahead,main_loop_lookahead+1,
    //   main_lig_loop,main_lig_loop+1,main_lig_loop+2,
    //   append_normal_space,exit;
    // var@!t:integer; {general-purpose temporary variable}
    // begin if every_job<>null then begin_token_list(every_job,every_job_text);
    if every_job!(globals) != null {
        begin_token_list(globals, every_job!(globals), every_job_text);
    }
    // big_switch: get_x_token;@/
    crate::region_backward_label! {
    'big_switch <-
    {
    get_x_token(globals)?;
    // reswitch: @<Give diagnostic information, if requested@>;
    crate::region_backward_label! {
    'reswitch <-
    {
    crate::region_forward_label! {
    |'append_normal_space|
    {
    crate::region_forward_label! {
    |'main_loop|
    {
    crate::section_1031::Give_diagnostic_information_if_requested!(globals, 'big_switch);
    // case abs(mode)+cur_cmd of
    let abs_mode_plus_cur_cmd = mode!(globals).get().abs() as u16 + globals.cur_cmd as u16;
    crate::trace_expr_verbose!("abs(mode)+cur_cmd={}", abs_mode_plus_cur_cmd);
    // hmode+letter,hmode+other_char,hmode+char_given: goto main_loop;
    if abs_mode_plus_cur_cmd == hmode as u16 + letter as u16 ||
        abs_mode_plus_cur_cmd == hmode as u16 + other_char as u16 ||
        abs_mode_plus_cur_cmd == hmode as u16 + char_given as u16 {
        crate::goto_forward_label!('main_loop);
    }
    // hmode+char_num: begin scan_char_num; cur_chr:=cur_val; goto main_loop;@+end;
    else if abs_mode_plus_cur_cmd == hmode as u16 + char_num as u16 {
        todo!();
    }
    // hmode+no_boundary: begin get_x_token;
    else if abs_mode_plus_cur_cmd == hmode as u16 + no_boundary as u16 {
        get_x_token(globals)?;
        // if (cur_cmd=letter)or(cur_cmd=other_char)or(cur_cmd=char_given)or
        //  (cur_cmd=char_num) then cancel_boundary:=true;
        if globals.cur_cmd == letter || globals.cur_cmd == other_char || globals.cur_cmd == char_given ||
            globals.cur_cmd == char_num {
            globals.cancel_boundary = true;
        }
        // goto reswitch;
        crate::goto_backward_label!('reswitch);
        // end;
    }
    // hmode+spacer: if space_factor=1000 then goto append_normal_space
    else if abs_mode_plus_cur_cmd == hmode as u16 + spacer as u16 {
        if space_factor!(globals) == 1000 {
            crate::goto_forward_label!('append_normal_space);
        } else {
            // else app_space;
            app_space(globals)?;

            use crate::section_1043::app_space;
        }
    }
    // hmode+ex_space,mmode+ex_space: goto append_normal_space;
    else if abs_mode_plus_cur_cmd == hmode as u16 + ex_space as u16 ||
        abs_mode_plus_cur_cmd == mmode as u16 + ex_space as u16 {
        crate::goto_forward_label!('append_normal_space);
    }
    // @t\4@>@<Cases of |main_control| that are not part of the inner loop@>@;
    else {
        crate::section_1045::Cases_of_main_control_that_are_not_part_of_the_inner_loop!(
            globals, abs_mode_plus_cur_cmd
        );
        // end; {of the big |case| statement}
    }
    /// end of the big `case` statement
    const _ : () = ();
    // goto big_switch;
    crate::goto_backward_label!('big_switch);
    }
    'main_loop <-
    }
    // main_loop:@<Append character |cur_chr| and the following characters (if~any)
    //   to the current hlist in the current font; |goto reswitch| when
    //   a non-character has been fetched@>;
    crate::section_1034::Append_character_cur_chr_and_the_following_characters_if_any_to_the_current_hlist_in_the_current_font__goto_reswitch_when_a_non_character_has_been_fetched!
        (globals, 'reswitch, 'big_switch);
    }
    'append_normal_space <-
    }
    // append_normal_space:@<Append a normal inter-word space to the current list,
    //   then |goto big_switch|@>;
     crate::section_1041::Append_a_normal_inter_word_space_to_the_current_list__then_goto_big_switch!
        (globals, 'big_switch);
    }
    |'reswitch|}
    }
    |'big_switch|}
    // exit:end;
    crate::return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0207::*;
use crate::section_0208::*;
use crate::section_0211::*;
use crate::section_0213::mode;
use crate::section_0213::space_factor;
use crate::section_0230::every_job;
use crate::section_0307::every_job_text;
use crate::section_0323::begin_token_list;
use crate::section_0380::get_x_token;
