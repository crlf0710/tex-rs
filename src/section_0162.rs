//! @* \[11] Memory layout.
//! Some areas of |mem| are dedicated to fixed usage, since static allocation is
//! more efficient than dynamic allocation when we can get away with it. For
//! example, locations |mem_bot| to |mem_bot+3| are always used to store the
//! specification for glue that is `\.{0pt plus 0pt minus 0pt}'. The
//! following macro definitions accomplish the static allocation by giving
//! symbolic names to the fixed positions. Static variable-size nodes appear
//! in locations |mem_bot| through |lo_mem_stat_max|, and static single-word nodes
//! appear in locations |hi_mem_stat_min| through |mem_top|, inclusive. It is
//! harmless to let |lig_trick| and |garbage| share the same location of |mem|.
//
// @d zero_glue==mem_bot {specification for \.{0pt plus 0pt minus 0pt}}
/// specification for `0pt plus 0pt minus 0pt`
pub(crate) const zero_glue: pointer = mem_bot;
// @d fil_glue==zero_glue+glue_spec_size {\.{0pt plus 1fil minus 0pt}}
/// `0pt plus 1fil minus 0pt`
pub(crate) const fil_glue: pointer = zero_glue + glue_spec_size as pointer;
// @d fill_glue==fil_glue+glue_spec_size {\.{0pt plus 1fill minus 0pt}}
/// `0pt plus 1fill minus 0pt`
pub(crate) const fill_glue: pointer = fil_glue + glue_spec_size as pointer;
// @d ss_glue==fill_glue+glue_spec_size {\.{0pt plus 1fil minus 1fil}}
/// `0pt plus 1fil minus 1fil`
pub(crate) const ss_glue: pointer = fill_glue + glue_spec_size as pointer;
// @d fil_neg_glue==ss_glue+glue_spec_size {\.{0pt plus -1fil minus 0pt}}
/// `0pt plus -1fil minus 0pt`
pub(crate) const fil_neg_glue: pointer = ss_glue + glue_spec_size as pointer;
// @d lo_mem_stat_max==fil_neg_glue+glue_spec_size-1 {largest statically
//   allocated word in the variable-size |mem|}
/// largest statically allocated word in the variable-size `mem`
pub(crate) const lo_mem_stat_max: pointer = fil_neg_glue + glue_spec_size as pointer - 1;
// @#
// @d page_ins_head==mem_top {list of insertion data for current page}
// @d contrib_head==mem_top-1 {vlist of items not yet on current page}
/// vlist of items not yet on current page
pub(crate) const contrib_head: pointer = mem_top - 1;
// @d page_head==mem_top-2 {vlist for current page}
// @d temp_head==mem_top-3 {head of a temporary list of some kind}
/// head of a temporary list of some kind
pub(crate) const temp_head: pointer = mem_top - 3;
// @d hold_head==mem_top-4 {head of a temporary list of another kind}
// @d adjust_head==mem_top-5 {head of adjustment list returned by |hpack|}
// @d active==mem_top-7 {head of active list in |line_break|, needs two words}
/// head of active list in `line_break`, needs two words
pub(crate) const active: pointer = mem_top - 7;
// @d align_head==mem_top-8 {head of preamble list for alignments}
// @d end_span==mem_top-9 {tail of spanned-width lists}
// @d omit_template==mem_top-10 {a constant token list}
/// a constant token list
pub(crate) const omit_template: pointer = mem_top - 10;
// @d null_list==mem_top-11 {permanently empty list}
// @d lig_trick==mem_top-12 {a ligature masquerading as a |char_node|}
// @d garbage==mem_top-12 {used for scrap information}
/// used for scrap information
pub(crate) const garbage: pointer = mem_top - 12;
// @d backup_head==mem_top-13 {head of token list built by |scan_keyword|}
/// head of token list built by `scan_keyword`
pub(crate) const backup_head: pointer = mem_top - 13;
// @d hi_mem_stat_min==mem_top-13 {smallest statically allocated word in
//   the one-word |mem|}
/// smallest statically allocated word in the one-word `mem`
pub(crate) const hi_mem_stat_min: pointer = mem_top - 13;
// @d hi_mem_stat_usage=14 {the number of one-word nodes always present}

use crate::section_0012::mem_top;
use crate::section_0012::mem_bot;

use crate::section_0115::pointer;
use crate::section_0150::glue_spec_size;
