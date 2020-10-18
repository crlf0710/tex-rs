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
pub(crate) fn main_control(globals: &mut TeXGlobals) {
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
    get_x_token(globals);
    // reswitch: @<Give diagnostic information, if requested@>;
    region_backward_label! {
    'reswitch <-
    {
    Give_diagnostic_information_if_requested!(globals);
    // case abs(mode)+cur_cmd of
    // hmode+letter,hmode+other_char,hmode+char_given: goto main_loop;
    // hmode+char_num: begin scan_char_num; cur_chr:=cur_val; goto main_loop;@+end;
    // hmode+no_boundary: begin get_x_token;
    //   if (cur_cmd=letter)or(cur_cmd=other_char)or(cur_cmd=char_given)or
    //    (cur_cmd=char_num) then cancel_boundary:=true;
    //   goto reswitch;
    //   end;
    // hmode+spacer: if space_factor=1000 then goto append_normal_space
    //   else app_space;
    // hmode+ex_space,mmode+ex_space: goto append_normal_space;
    // @t\4@>@<Cases of |main_control| that are not part of the inner loop@>@;
    // end; {of the big |case| statement}
    // goto big_switch;
    goto_backward_label!('big_switch);
    // main_loop:@<Append character |cur_chr| and the following characters (if~any)
    //   to the current hlist in the current font; |goto reswitch| when
    //   a non-character has been fetched@>;
    // append_normal_space:@<Append a normal inter-word space to the current list,
    //   then |goto big_switch|@>;
    }
    |'reswitch|}
    }
    |'big_switch|}
    // exit:end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0380::get_x_token;
