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
pub(crate) fn main_control(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    // label big_switch,reswitch,main_loop,main_loop_wrapup,
    //   main_loop_move,main_loop_move+1,main_loop_move+2,main_loop_move_lig,
    //   main_loop_lookahead,main_loop_lookahead+1,
    //   main_lig_loop,main_lig_loop+1,main_lig_loop+2,
    //   append_normal_space,exit;
    // var@!t:integer; {general-purpose temporary variable}
    // begin if every_job<>null then begin_token_list(every_job,every_job_text);
    // big_switch: get_x_token;@/
    region_backward_label! {
    'big_switch <-
    {
    get_x_token(globals)?;
    // reswitch: @<Give diagnostic information, if requested@>;
    region_backward_label! {
    'reswitch <-
    {
    region_forward_label! {
    |'append_normal_space|
    {
    region_forward_label! {
    |'main_loop|
    {
    Give_diagnostic_information_if_requested!(globals, 'big_switch);
    // case abs(mode)+cur_cmd of
    let abs_mode_plus_cur_cmd = mode!(globals).get().abs() as u16 + globals.cur_cmd as u16;
    trace_expr!("abs(mode)+cur_cmd={}", abs_mode_plus_cur_cmd);
    // hmode+letter,hmode+other_char,hmode+char_given: goto main_loop;
    if abs_mode_plus_cur_cmd == hmode as u16 + letter as u16 ||
        abs_mode_plus_cur_cmd == hmode as u16 + other_char as u16 ||
        abs_mode_plus_cur_cmd == hmode as u16 + char_given as u16 {
        goto_forward_label!('main_loop);
    }
    // hmode+char_num: begin scan_char_num; cur_chr:=cur_val; goto main_loop;@+end;
    else if abs_mode_plus_cur_cmd == hmode as u16 + char_num as u16 {
        todo!();
    }
    // hmode+no_boundary: begin get_x_token;
    else if abs_mode_plus_cur_cmd == hmode as u16 + no_boundary as u16 {
        todo!();
        // if (cur_cmd=letter)or(cur_cmd=other_char)or(cur_cmd=char_given)or
        //  (cur_cmd=char_num) then cancel_boundary:=true;
        // goto reswitch;
        // end;
    }
    // hmode+spacer: if space_factor=1000 then goto append_normal_space
    else if abs_mode_plus_cur_cmd == hmode as u16 + spacer as u16 {
        todo!();
        // else app_space;
    }
    // hmode+ex_space,mmode+ex_space: goto append_normal_space;
    else if abs_mode_plus_cur_cmd == hmode as u16 + ex_space as u16 ||
        abs_mode_plus_cur_cmd == mmode as u16 + ex_space as u16 {
        todo!();
    }
    // @t\4@>@<Cases of |main_control| that are not part of the inner loop@>@;
    else {
        Cases_of_main_control_that_are_not_part_of_the_inner_loop!(
            globals, abs_mode_plus_cur_cmd
        );
        // end; {of the big |case| statement}
    }
    /// end of the big `case` statement
    const _ : () = ();
    // goto big_switch;
    goto_backward_label!('big_switch);
    }
    'main_loop <-
    }
    // main_loop:@<Append character |cur_chr| and the following characters (if~any)
    //   to the current hlist in the current font; |goto reswitch| when
    //   a non-character has been fetched@>;
    }
    'append_normal_space <-
    }
    // append_normal_space:@<Append a normal inter-word space to the current list,
    //   then |goto big_switch|@>;
    }
    |'reswitch|}
    }
    |'big_switch|}
    // exit:end;
    return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0207::*;
use crate::section_0208::*;
use crate::section_0211::*;
use crate::section_0380::get_x_token;
use crate::section_0081::JumpOutToEndOfTEX;
