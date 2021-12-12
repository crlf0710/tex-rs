//! @ We also need to compute the change in style between mlists and their
//! subsidiaries. The following macros define the subsidiary style for
//! an overlined nucleus (|cramped_style|), for a subscript or a superscript
//! (|sub_style| or |sup_style|), or for a numerator or denominator (|num_style|
//! or |denom_style|).
//
// @d cramped_style(#)==2*(# div 2)+cramped {cramp the style}
// @d sub_style(#)==2*(# div 4)+script_style+cramped {smaller and cramped}
// @d sup_style(#)==2*(# div 4)+script_style+(# mod 2) {smaller}
/// smaller
pub(crate) macro sup_style($v:expr) {
    2 * ($v.get() / 4)
        + crate::section_0688::style_node_subtype::script_style.get()
        + ($v.get() % 2)
}
// @d num_style(#)==#+2-2*(# div 6) {smaller unless already script-script}
// @d denom_style(#)==2*(# div 2)+cramped+2-2*(# div 6) {smaller, cramped}
//
