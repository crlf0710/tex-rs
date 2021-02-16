//! @ Region 5 of |eqtb| contains the integer parameters and registers defined
//! here, as well as the |del_code| table. The latter table differs from the
//! |cat_code..math_code| tables that precede it, since delimiter codes are
//! fullword integers while the other kinds of codes occupy at most a
//! halfword. This is what makes region~5 different from region~4. We will
//! store the |eq_level| information in an auxiliary array of quarterwords
//! that will be defined later.
//
// @d pretolerance_code=0 {badness tolerance before hyphenation}
/// badness tolerance before hyphenation
pub(crate) const pretolerance_code: quarterword = 0;
// @d tolerance_code=1 {badness tolerance after hyphenation}
/// badness tolerance after hyphenation
pub(crate) const tolerance_code: quarterword = 1;
// @d line_penalty_code=2 {added to the badness of every line}
/// added to the badness of every line
pub(crate) const line_penalty_code: quarterword = 2;
// @d hyphen_penalty_code=3 {penalty for break after discretionary hyphen}
/// penalty for break after discretionary hyphen
pub(crate) const hyphen_penalty_code: quarterword = 3;
// @d ex_hyphen_penalty_code=4 {penalty for break after explicit hyphen}
/// penalty for break after explicit hyphen
pub(crate) const ex_hyphen_penalty_code: quarterword = 4;
// @d club_penalty_code=5 {penalty for creating a club line}
/// penalty for creating a club line
pub(crate) const club_penalty_code: quarterword = 5;
// @d widow_penalty_code=6 {penalty for creating a widow line}
/// penalty for creating a widow line
pub(crate) const widow_penalty_code: quarterword = 6;
// @d display_widow_penalty_code=7 {ditto, just before a display}
/// penalty for creating a widow line before a display
pub(crate) const display_widow_penalty_code: quarterword = 7;
// @d broken_penalty_code=8 {penalty for breaking a page at a broken line}
/// penalty for breaking a page at a broken line
pub(crate) const broken_penalty_code: quarterword = 8;
// @d bin_op_penalty_code=9 {penalty for breaking after a binary operation}
/// penalty for breaking after a binary operation
pub(crate) const bin_op_penalty_code: quarterword = 9;
// @d rel_penalty_code=10 {penalty for breaking after a relation}
/// penalty for breaking after a relation
pub(crate) const rel_penalty_code: quarterword = 10;
// @d pre_display_penalty_code=11
//   {penalty for breaking just before a displayed formula}
/// penalty for breaking just before a displayed formula
pub(crate) const pre_display_penalty_code: quarterword = 11;
// @d post_display_penalty_code=12
//   {penalty for breaking just after a displayed formula}
/// penalty for breaking just after a displayed formula
pub(crate) const post_display_penalty_code: quarterword = 12;
// @d inter_line_penalty_code=13 {additional penalty between lines}
/// additional penalty between lines
pub(crate) const inter_line_penalty_code: quarterword = 13;
// @d double_hyphen_demerits_code=14 {demerits for double hyphen break}
/// demerits for double hyphen break
pub(crate) const double_hyphen_demerits_code: quarterword = 14;
// @d final_hyphen_demerits_code=15 {demerits for final hyphen break}
/// demerits for final hyphen break
pub(crate) const final_hyphen_demerits_code: quarterword = 15;
// @d adj_demerits_code=16 {demerits for adjacent incompatible lines}
/// demerits for adjacent incompatible lines
pub(crate) const adj_demerits_code: quarterword = 16;
// @d mag_code=17 {magnification ratio}
/// magnification ratio
pub(crate) const mag_code: quarterword = 17;
// @d delimiter_factor_code=18 {ratio for variable-size delimiters}
/// ratio for variable-size delimiters
pub(crate) const delimiter_factor_code: quarterword = 18;
// @d looseness_code=19 {change in number of lines for a paragraph}
/// change in number of lines for a paragraph
pub(crate) const looseness_code: quarterword = 19;
// @d time_code=20 {current time of day}
/// current time of day
pub(crate) const time_code: quarterword = 20;
// @d day_code=21 {current day of the month}
/// current day of the month
pub(crate) const day_code: quarterword = 21;
// @d month_code=22 {current month of the year}
/// current month of the year
pub(crate) const month_code: quarterword = 22;
// @d year_code=23 {current year of our Lord}
/// current year of our Lord
pub(crate) const year_code: quarterword = 23;
// @d show_box_breadth_code=24 {nodes per level in |show_box|}
/// nodes per level in `show_box`
pub(crate) const show_box_breadth_code: quarterword = 24;
// @d show_box_depth_code=25 {maximum level in |show_box|}
/// maximum level in `show_box`
pub(crate) const show_box_depth_code: quarterword = 25;
// @d hbadness_code=26 {hboxes exceeding this badness will be shown by |hpack|}
/// hboxes exceeding this badness will be shown by `hpack`
pub(crate) const hbadness_code: quarterword = 26;
// @d vbadness_code=27 {vboxes exceeding this badness will be shown by |vpack|}
/// vboxes exceeding this badness will be shown by `vpack`
pub(crate) const vbadness_code: quarterword = 27;
// @d pausing_code=28 {pause after each line is read from a file}
/// pause after each line is read from a file
pub(crate) const pausing_code: quarterword = 28;
// @d tracing_online_code=29 {show diagnostic output on terminal}
/// show diagnostic output on terminal
pub(crate) const tracing_online_code: quarterword = 29;
// @d tracing_macros_code=30 {show macros as they are being expanded}
/// show macros as they are being expanded
pub(crate) const tracing_macros_code: quarterword = 30;
// @d tracing_stats_code=31 {show memory usage if \TeX\ knows it}
/// show memory usage if `TeX` knows it
pub(crate) const tracing_stats_code: quarterword = 31;
// @d tracing_paragraphs_code=32 {show line-break calculations}
/// show line-break calculations
pub(crate) const tracing_paragraphs_code: quarterword = 32;
// @d tracing_pages_code=33 {show page-break calculations}
/// show page-break calculations
pub(crate) const tracing_pages_code: quarterword = 33;
// @d tracing_output_code=34 {show boxes when they are shipped out}
/// show boxes when they are shipped out
pub(crate) const tracing_output_code: quarterword = 34;
// @d tracing_lost_chars_code=35 {show characters that aren't in the font}
/// show characters that aren't in the font
pub(crate) const tracing_lost_chars_code: quarterword = 35;
// @d tracing_commands_code=36 {show command codes at |big_switch|}
/// show command codes at `big_switch`
pub(crate) const tracing_commands_code: quarterword = 36;
// @d tracing_restores_code=37 {show equivalents when they are restored}
/// show equivalents when they are restored
pub(crate) const tracing_restores_code: quarterword = 37;
// @d uc_hyph_code=38 {hyphenate words beginning with a capital letter}
/// hyphenate words beginning with a capital letter
pub(crate) const uc_hyph_code: quarterword = 38;
// @d output_penalty_code=39 {penalty found at current page break}
/// penalty found at current page break
pub(crate) const output_penalty_code: quarterword = 39;
// @d max_dead_cycles_code=40 {bound on consecutive dead cycles of output}
/// bound on consecutive dead cycles of output
pub(crate) const max_dead_cycles_code: quarterword = 40;
// @d hang_after_code=41 {hanging indentation changes after this many lines}
/// hanging indentation changes after this many lines
pub(crate) const hang_after_code: quarterword = 41;
// @d floating_penalty_code=42 {penalty for insertions held over after a split}
/// penalty for insertions held over after a split
pub(crate) const floating_penalty_code: quarterword = 42;
// @d global_defs_code=43 {override \.{\\global} specifications}
/// override `\global` specifications
pub(crate) const global_defs_code: quarterword = 43;
// @d cur_fam_code=44 {current family}
/// current family
pub(crate) const cur_fam_code: quarterword = 44;
// @d escape_char_code=45 {escape character for token output}
/// escape character for token output
pub(crate) const escape_char_code: quarterword = 45;
// @d default_hyphen_char_code=46 {value of \.{\\hyphenchar} when a font is loaded}
/// value of `\hyphenchar` when a font is loaded
pub(crate) const default_hyphen_char_code: quarterword = 46;
// @d default_skew_char_code=47 {value of \.{\\skewchar} when a font is loaded}
/// value of `\skewchar` when a font is loaded
pub(crate) const default_skew_char_code: quarterword = 47;
// @d end_line_char_code=48 {character placed at the right end of the buffer}
/// character placed at the right end of the buffer
pub(crate) const end_line_char_code: quarterword = 48;
// @d new_line_char_code=49 {character that prints as |print_ln|}
/// character that prints as `print_ln`
pub(crate) const new_line_char_code: quarterword = 49;
// @d language_code=50 {current hyphenation table}
/// current hyphenation table
pub(crate) const language_code: quarterword = 50;
// @d left_hyphen_min_code=51 {minimum left hyphenation fragment size}
/// minimum left hyphenation fragment size
pub(crate) const left_hyphen_min_code: quarterword = 51;
// @d right_hyphen_min_code=52 {minimum right hyphenation fragment size}
/// minimum right hyphenation fragment size
pub(crate) const right_hyphen_min_code: quarterword = 52;
// @d holding_inserts_code=53 {do not remove insertion nodes from \.{\\box255}}
/// do not remove insertion nodes from `\box255`
pub(crate) const holding_inserts_code: quarterword = 53;
// @d error_context_lines_code=54 {maximum intermediate line pairs shown}
/// maximum intermediate line pairs shown
pub(crate) const error_context_lines_code: quarterword = 54;
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
macro_rules! count {
    ($globals:expr, $val:expr) => {
        $globals.eqtb[crate::section_0236::count_base as crate::section_0115::pointer
            + $val as crate::section_0115::pointer][crate::section_0113::MEMORY_WORD_INT]
    };
}
// @d int_par(#)==eqtb[int_base+#].int {an integer parameter}
/// an integer parameter
macro_rules! int_par {
    ($globals:expr, $val:expr) => {
        $globals.eqtb[crate::section_0230::int_base as crate::section_0115::pointer
            + $val as crate::section_0115::pointer][crate::section_0113::MEMORY_WORD_INT]
    };
}
// @d pretolerance==int_par(pretolerance_code)
macro_rules! pretolerance {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::pretolerance_code)
    };
}
// @d tolerance==int_par(tolerance_code)
macro_rules! tolerance {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::tolerance_code)
    };
}
// @d line_penalty==int_par(line_penalty_code)
macro_rules! line_penalty {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::line_penalty_code)
    };
}
// @d hyphen_penalty==int_par(hyphen_penalty_code)
// @d ex_hyphen_penalty==int_par(ex_hyphen_penalty_code)
// @d club_penalty==int_par(club_penalty_code)
macro_rules! club_penalty {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::club_penalty_code)
    };
}
// @d widow_penalty==int_par(widow_penalty_code)
macro_rules! widow_penalty {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::widow_penalty_code)
    };
}
// @d display_widow_penalty==int_par(display_widow_penalty_code)
// @d broken_penalty==int_par(broken_penalty_code)
macro_rules! broken_penalty {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::broken_penalty_code)
    };
}
// @d bin_op_penalty==int_par(bin_op_penalty_code)
// @d rel_penalty==int_par(rel_penalty_code)
// @d pre_display_penalty==int_par(pre_display_penalty_code)
// @d post_display_penalty==int_par(post_display_penalty_code)
// @d inter_line_penalty==int_par(inter_line_penalty_code)
macro_rules! inter_line_penalty {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::inter_line_penalty_code)
    };
}
// @d double_hyphen_demerits==int_par(double_hyphen_demerits_code)
// @d final_hyphen_demerits==int_par(final_hyphen_demerits_code)
// @d adj_demerits==int_par(adj_demerits_code)
macro_rules! adj_demerits {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::adj_demerits_code)
    };
}
// @d mag==int_par(mag_code)
macro_rules! mag {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::mag_code)
    };
}
// @d delimiter_factor==int_par(delimiter_factor_code)
// @d looseness==int_par(looseness_code)
macro_rules! looseness {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::looseness_code)
    };
}
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
macro_rules! show_box_breadth {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::show_box_breadth_code)
    };
}
// @d show_box_depth==int_par(show_box_depth_code)
macro_rules! show_box_depth {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::show_box_depth_code)
    };
}
// @d hbadness==int_par(hbadness_code)
macro_rules! hbadness {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::hbadness_code)
    };
}
// @d vbadness==int_par(vbadness_code)
macro_rules! vbadness {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::vbadness_code)
    };
}
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
macro_rules! max_dead_cycles {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::max_dead_cycles_code)
    };
}
// @d hang_after==int_par(hang_after_code)
macro_rules! hang_after {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::hang_after_code)
    };
}
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
macro_rules! default_hyphen_char {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::default_hyphen_char_code)
    };
}
// @d default_skew_char==int_par(default_skew_char_code)
macro_rules! default_skew_char {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::default_skew_char_code)
    };
}
// @d end_line_char==int_par(end_line_char_code)
macro_rules! end_line_char {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::end_line_char_code)
    };
}
// @d new_line_char==int_par(new_line_char_code)
macro_rules! new_line_char {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::new_line_char_code)
    };
}
// @d language==int_par(language_code)
macro_rules! language {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::language_code)
    };
}
// @d left_hyphen_min==int_par(left_hyphen_min_code)
macro_rules! left_hyphen_min {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::left_hyphen_min_code)
    };
}
// @d right_hyphen_min==int_par(right_hyphen_min_code)
macro_rules! right_hyphen_min {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::right_hyphen_min_code)
    };
}
// @d holding_inserts==int_par(holding_inserts_code)
macro_rules! holding_inserts {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::holding_inserts_code)
    };
}
// @d error_context_lines==int_par(error_context_lines_code)
macro_rules! error_context_lines {
    ($globals:expr) => {
        int_par!($globals, crate::section_0236::error_context_lines_code)
    };
}
//
// @<Assign the values |depth_threshold:=show_box_depth|...@>=
macro_rules! Assign_the_values_depth_threshold_from_show_box_depth_and_breadth_max_show_box_breadth {
    ($globals:expr) => {{
        // depth_threshold:=show_box_depth;
        $globals.depth_threshold = show_box_depth!($globals);
        // breadth_max:=show_box_breadth
        $globals.breadth_max = show_box_breadth!($globals);
    }}
}

use crate::pascal::word;
use crate::section_0113::halfword;
use crate::section_0113::quarterword;
use crate::section_0230::int_base_TYPENUM;
use typenum::Unsigned;
use typenum::U256;
