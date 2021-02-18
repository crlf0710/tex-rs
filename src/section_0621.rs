//! ` `
// @<Change font |dvi_f| to |f|@>=
macro_rules! Change_font_dvi_f_to_f {
    ($globals:expr) => {{
        // begin if not font_used[f] then
        if !$globals.font_used[$globals.ship_out_f] {
            // begin dvi_font_def(f); font_used[f]:=true;
            dvi_font_def($globals, $globals.ship_out_f);
            $globals.font_used[$globals.ship_out_f] = true;
            // end;
        }
        // if f<=64+font_base then dvi_out(f-font_base-1+fnt_num_0)
        if $globals.ship_out_f.get() as integer <= 64 + font_base as integer {
            dvi_out!(
                $globals,
                $globals.ship_out_f.get() as integer - font_base as integer - 1
                    + fnt_num_0.byte() as integer
            );
        }
        // else  begin dvi_out(fnt1); dvi_out(f-font_base-1);
        else {
            dvi_out!($globals, fnt1.byte());
            dvi_out!(
                $globals,
                $globals.ship_out_f.get() as integer - font_base as integer - 1
            );
            // end;
        }
        // dvi_f:=f;
        $globals.dvi_f = $globals.ship_out_f;
        // end
        use crate::section_0012::font_base;
        use crate::section_0586::fnt1;
        use crate::section_0586::fnt_num_0;
        use crate::section_0602::dvi_font_def;
    }};
}
