//! @ ` `

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct dvi_command(u8);

impl dvi_command {
    pub(crate) fn byte(self) -> u8 {
        self.0
    }
}

// @d set_char_0=0 {typeset character 0 and move right}
/// typeset character 0 and move right
pub(crate) const set_char_0: dvi_command = dvi_command(0);
// @d set1=128 {typeset a character and move right}
/// typeset a character and move right
pub(crate) const set1: dvi_command = dvi_command(128);
#[cfg(feature = "unicode_support")]
pub(crate) const set4: dvi_command = dvi_command(131);
// @d set_rule=132 {typeset a rule and move right}
/// typeset a rule and move right
pub(crate) const set_rule: dvi_command = dvi_command(132);
// @d put_rule=137 {typeset a rule}
/// typeset a rule
pub(crate) const put_rule: dvi_command = dvi_command(137);
// @d nop=138 {no operation}
/// no operation
pub(crate) const nop: dvi_command = dvi_command(138);
// @d bop=139 {beginning of page}
/// beginning of page
pub(crate) const bop: dvi_command = dvi_command(139);
// @d eop=140 {ending of page}
/// ending of page
pub(crate) const eop: dvi_command = dvi_command(140);
// @d push=141 {save the current positions}
/// save the current positions
pub(crate) const push: dvi_command = dvi_command(141);
// @d pop=142 {restore previous positions}
/// restore previous positions
pub(crate) const pop: dvi_command = dvi_command(142);
// @d right1=143 {move right}
/// move right
pub(crate) const right1: dvi_command = dvi_command(143);
// @d w0=147 {move right by |w|}
/// move right by `w`
pub(crate) const w0: dvi_command = dvi_command(147);
// @d w1=148 {move right and set |w|}
/// move right and set `w`
pub(crate) const w1: dvi_command = dvi_command(148);
// @d x0=152 {move right by |x|}
/// move right by `x`
pub(crate) const x0: dvi_command = dvi_command(152);
// @d x1=153 {move right and set |x|}
/// move right and set `x`
pub(crate) const x1: dvi_command = dvi_command(153);
// @d down1=157 {move down}
/// move down
pub(crate) const down1: dvi_command = dvi_command(157);
// @d y0=161 {move down by |y|}
/// move down by `y`
pub(crate) const y0: dvi_command = dvi_command(161);
// @d y1=162 {move down and set |y|}
/// move down and set `y`
pub(crate) const y1: dvi_command = dvi_command(162);
// @d z0=166 {move down by |z|}
/// move down by `z`
pub(crate) const z0: dvi_command = dvi_command(166);
// @d z1=167 {move down and set |z|}
/// move down and set `z`
pub(crate) const z1: dvi_command = dvi_command(167);
// @d fnt_num_0=171 {set current font to 0}
/// set current font to 0
pub(crate) const fnt_num_0: dvi_command = dvi_command(171);
// @d fnt1=235 {set current font}
/// set current font
pub(crate) const fnt1: dvi_command = dvi_command(235);
// @d xxx1=239 {extension to \.{DVI} primitives}
/// extension to `DVI` primitives
pub(crate) const xxx1: dvi_command = dvi_command(239);
// @d xxx4=242 {potentially long extension to \.{DVI} primitives}
/// potentially long extension to `DVI` primitives
pub(crate) const xxx4: dvi_command = dvi_command(242);
// @d fnt_def1=243 {define the meaning of a font number}
/// define the meaning of a font number
pub(crate) const fnt_def1: dvi_command = dvi_command(243);
// @d pre=247 {preamble}
/// preamble
pub(crate) const pre: dvi_command = dvi_command(247);
// @d post=248 {postamble beginning}
/// postamble beginning
pub(crate) const post: dvi_command = dvi_command(248);
// @d post_post=249 {postamble ending}
/// postamble beginning
pub(crate) const post_post: dvi_command = dvi_command(249);

crate::migration_complete!();
