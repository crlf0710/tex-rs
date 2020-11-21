//! @ Region 5 of |eqtb| contains the integer parameters and registers defined
//! here, as well as the |del_code| table. The latter table differs from the
//! |cat_code..math_code| tables that precede it, since delimiter codes are
//! fullword integers while the other kinds of codes occupy at most a
//! halfword. This is what makes region~5 different from region~4. We will
//! store the |eq_level| information in an auxiliary array of quarterwords
//! that will be defined later.
//
// @d pretolerance_code=0 {badness tolerance before hyphenation}
// @d tolerance_code=1 {badness tolerance after hyphenation}
// @d line_penalty_code=2 {added to the badness of every line}
// @d hyphen_penalty_code=3 {penalty for break after discretionary hyphen}
// @d ex_hyphen_penalty_code=4 {penalty for break after explicit hyphen}
// @d club_penalty_code=5 {penalty for creating a club line}
// @d widow_penalty_code=6 {penalty for creating a widow line}
// @d display_widow_penalty_code=7 {ditto, just before a display}
// @d broken_penalty_code=8 {penalty for breaking a page at a broken line}
// @d bin_op_penalty_code=9 {penalty for breaking after a binary operation}
// @d rel_penalty_code=10 {penalty for breaking after a relation}
// @d pre_display_penalty_code=11
//   {penalty for breaking just before a displayed formula}
// @d post_display_penalty_code=12
//   {penalty for breaking just after a displayed formula}
// @d inter_line_penalty_code=13 {additional penalty between lines}
// @d double_hyphen_demerits_code=14 {demerits for double hyphen break}
// @d final_hyphen_demerits_code=15 {demerits for final hyphen break}
// @d adj_demerits_code=16 {demerits for adjacent incompatible lines}
// @d mag_code=17 {magnification ratio}
// @d delimiter_factor_code=18 {ratio for variable-size delimiters}
// @d looseness_code=19 {change in number of lines for a paragraph}
// @d time_code=20 {current time of day}
/// current time of day
pub(crate) const time_code: halfword = 20;
// @d day_code=21 {current day of the month}
/// current day of the month
pub(crate) const day_code: halfword = 21;
// @d month_code=22 {current month of the year}
/// current month of the year
pub(crate) const month_code: halfword = 22;
// @d year_code=23 {current year of our Lord}
/// current year of our Lord
pub(crate) const year_code: halfword = 23;
// @d show_box_breadth_code=24 {nodes per level in |show_box|}
// @d show_box_depth_code=25 {maximum level in |show_box|}
// @d hbadness_code=26 {hboxes exceeding this badness will be shown by |hpack|}
// @d vbadness_code=27 {vboxes exceeding this badness will be shown by |vpack|}
// @d pausing_code=28 {pause after each line is read from a file}
// @d tracing_online_code=29 {show diagnostic output on terminal}
/// show diagnostic output on terminal
pub(crate) const tracing_online_code: halfword = 29;
// @d tracing_macros_code=30 {show macros as they are being expanded}
/// show macros as they are being expanded
pub(crate) const tracing_macros_code: halfword = 30;
// @d tracing_stats_code=31 {show memory usage if \TeX\ knows it}
// @d tracing_paragraphs_code=32 {show line-break calculations}
// @d tracing_pages_code=33 {show page-break calculations}
// @d tracing_output_code=34 {show boxes when they are shipped out}
// @d tracing_lost_chars_code=35 {show characters that aren't in the font}
/// show characters that aren't in the font
pub(crate) const tracing_lost_chars_code: halfword = 35;
// @d tracing_commands_code=36 {show command codes at |big_switch|}
/// show command codes at `big_switch`
pub(crate) const tracing_commands_code: halfword = 36;
// @d tracing_restores_code=37 {show equivalents when they are restored}
// @d uc_hyph_code=38 {hyphenate words beginning with a capital letter}
// @d output_penalty_code=39 {penalty found at current page break}
// @d max_dead_cycles_code=40 {bound on consecutive dead cycles of output}
// @d hang_after_code=41 {hanging indentation changes after this many lines}
// @d floating_penalty_code=42 {penalty for insertions heldover after a split}
// @d global_defs_code=43 {override \.{\\global} specifications}
/// override `\global` specifications
pub(crate) const global_defs_code: halfword = 43;
// @d cur_fam_code=44 {current family}
// @d escape_char_code=45 {escape character for token output}
/// escape character for token output
pub(crate) const escape_char_code: halfword = 45;
// @d default_hyphen_char_code=46 {value of \.{\\hyphenchar} when a font is loaded}
// @d default_skew_char_code=47 {value of \.{\\skewchar} when a font is loaded}
// @d end_line_char_code=48 {character placed at the right end of the buffer}
/// character placed at the right end of the buffer
pub(crate) const end_line_char_code: halfword = 48;
// @d new_line_char_code=49 {character that prints as |print_ln|}
// @d language_code=50 {current hyphenation table}
// @d left_hyphen_min_code=51 {minimum left hyphenation fragment size}
// @d right_hyphen_min_code=52 {minimum right hyphenation fragment size}
// @d holding_inserts_code=53 {do not remove insertion nodes from \.{\\box255}}
// @d error_context_lines_code=54 {maximum intermediate line pairs shown}
// @d int_pars=55 {total number of integer parameters}
/// total number of integer parameters
pub(crate) type int_pars_TYPENUM = typenum::U55;
pub(crate) const int_pars: halfword = int_pars_TYPENUM::U16;
// @d count_base=int_base+int_pars {256 user \.{\\count} registers}
/// 256 user `\count` registers
pub(crate) type count_base_TYPENUM = typenum::op!(int_base_TYPENUM + int_pars_TYPENUM);
pub(crate) const count_base: word = count_base_TYPENUM::U32;
// @d del_code_base=count_base+256 {256 delimiter code mappings}
/// 256 delimiter code mappings
pub(crate) type del_code_base_TYPENUM = typenum::op!(count_base_TYPENUM + U256);
pub(crate) const del_code_base: word = del_code_base_TYPENUM::U32;
// @d dimen_base=del_code_base+256 {beginning of region 6}
/// beginning of region 6
pub(crate) type dimen_base_TYPENUM = typenum::op!(del_code_base_TYPENUM + U256);
pub(crate) const dimen_base: word = dimen_base_TYPENUM::U32;
// @#
// @d del_code(#)==eqtb[del_code_base+#].int
// @d count(#)==eqtb[count_base+#].int
// @d int_par(#)==eqtb[int_base+#].int {an integer parameter}
/// an integer parameter
macro_rules! int_par {
    ($globals:expr, $val:expr) => {
        $globals.eqtb[crate::section_0230::int_base as crate::section_0115::pointer
            + $val as crate::section_0115::pointer][crate::section_0113::MEMORY_WORD_INT]
    };
}
// @d pretolerance==int_par(pretolerance_code)
// @d tolerance==int_par(tolerance_code)
// @d line_penalty==int_par(line_penalty_code)
// @d hyphen_penalty==int_par(hyphen_penalty_code)
// @d ex_hyphen_penalty==int_par(ex_hyphen_penalty_code)
// @d club_penalty==int_par(club_penalty_code)
// @d widow_penalty==int_par(widow_penalty_code)
// @d display_widow_penalty==int_par(display_widow_penalty_code)
// @d broken_penalty==int_par(broken_penalty_code)
// @d bin_op_penalty==int_par(bin_op_penalty_code)
// @d rel_penalty==int_par(rel_penalty_code)
// @d pre_display_penalty==int_par(pre_display_penalty_code)
// @d post_display_penalty==int_par(post_display_penalty_code)
// @d inter_line_penalty==int_par(inter_line_penalty_code)
// @d double_hyphen_demerits==int_par(double_hyphen_demerits_code)
// @d final_hyphen_demerits==int_par(final_hyphen_demerits_code)
// @d adj_demerits==int_par(adj_demerits_code)
// @d mag==int_par(mag_code)
// @d delimiter_factor==int_par(delimiter_factor_code)
// @d looseness==int_par(looseness_code)
// @d time==int_par(time_code)
macro_rules! time {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::time_code)
    };
}
// @d day==int_par(day_code)
macro_rules! day {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::day_code)
    };
}
// @d month==int_par(month_code)
macro_rules! month {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::month_code)
    };
}
// @d year==int_par(year_code)
macro_rules! year {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::year_code)
    };
}
// @d show_box_breadth==int_par(show_box_breadth_code)
// @d show_box_depth==int_par(show_box_depth_code)
// @d hbadness==int_par(hbadness_code)
// @d vbadness==int_par(vbadness_code)
// @d pausing==int_par(pausing_code)
// @d tracing_online==int_par(tracing_online_code)
macro_rules! tracing_online {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::tracing_online_code)
    };
}
// @d tracing_macros==int_par(tracing_macros_code)
macro_rules! tracing_macros {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::tracing_macros_code)
    };
}
// @d tracing_stats==int_par(tracing_stats_code)
// @d tracing_paragraphs==int_par(tracing_paragraphs_code)
// @d tracing_pages==int_par(tracing_pages_code)
// @d tracing_output==int_par(tracing_output_code)
// @d tracing_lost_chars==int_par(tracing_lost_chars_code)
macro_rules! tracing_lost_chars {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::tracing_lost_chars_code)
    };
}
// @d tracing_commands==int_par(tracing_commands_code)
macro_rules! tracing_commands {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::tracing_commands_code)
    };
}
// @d tracing_restores==int_par(tracing_restores_code)
// @d uc_hyph==int_par(uc_hyph_code)
// @d output_penalty==int_par(output_penalty_code)
// @d max_dead_cycles==int_par(max_dead_cycles_code)
// @d hang_after==int_par(hang_after_code)
// @d floating_penalty==int_par(floating_penalty_code)
// @d global_defs==int_par(global_defs_code)
macro_rules! global_defs {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::global_defs_code)
    };
}
// @d cur_fam==int_par(cur_fam_code)
// @d escape_char==int_par(escape_char_code)
macro_rules! escape_char {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::escape_char_code)
    };
}
// @d default_hyphen_char==int_par(default_hyphen_char_code)
// @d default_skew_char==int_par(default_skew_char_code)
// @d end_line_char==int_par(end_line_char_code)
macro_rules! end_line_char {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::end_line_char_code)
    };
}
// @d new_line_char==int_par(new_line_char_code)
// @d language==int_par(language_code)
// @d left_hyphen_min==int_par(left_hyphen_min_code)
// @d right_hyphen_min==int_par(right_hyphen_min_code)
// @d holding_inserts==int_par(holding_inserts_code)
// @d error_context_lines==int_par(error_context_lines_code)
//
// @<Assign the values |depth_threshold:=show_box_depth|...@>=
// depth_threshold:=show_box_depth;
// breadth_max:=show_box_breadth

use crate::pascal::word;
use crate::section_0113::halfword;
use crate::section_0230::int_base_TYPENUM;
use typenum::Unsigned;
use typenum::U256;
