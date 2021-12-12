//! ` `

// @<Append inter-element spacing based on |r_type| and |t|@>=
pub(crate) macro Append_inter_element_spacing_based_on_r_type_and_t($globals:expr, $r_type:expr, $t:expr) {{
    // if r_type>0 then {not the first noad}
    if $r_type > 0 {
        /// not the first noad
        const _: () = ();
        /// temporary registers for list construction
        let x;
        // begin case so(str_pool[r_type*8+t+magic_offset]) of
        let spacing = crate::section_0764::MATH_SPACING[($r_type as isize * 8
            + $t as isize
            + crate::section_0765::MATH_SPACING_OFFSET)
            as usize];
        // "0": x:=0;
        if spacing == 0 {
            x = 0;
        }
        // "1": if cur_style<script_style then x:=thin_mu_skip_code@+else x:=0;
        else if spacing == 1 {
            todo!("1");
        }
        // "2": x:=thin_mu_skip_code;
        else if spacing == 2 {
            todo!("2");
        }
        // "3": if cur_style<script_style then x:=med_mu_skip_code@+else x:=0;
        else if spacing == 3 {
            todo!("3");
        }
        // "4": if cur_style<script_style then x:=thick_mu_skip_code@+else x:=0;
        else if spacing == 4 {
            todo!("4");
        }
        // othercases confusion("mlist4")
        else {
            confusion($globals, crate::strpool_str!("mlist4"))?;
            // @:this can't happen mlist4}{\quad mlist4@>
        }
        // endcases;
        // if x<>0 then
        if x != 0 {
            // begin y:=math_glue(glue_par(x),cur_mu);
            // z:=new_glue(y); glue_ref_count(y):=null; link(p):=z; p:=z;@/
            // subtype(z):=x+1; {store a symbolic subtype}
            // end;
            todo!("x != 0");
        }
        // end
    }
    use crate::section_0095::confusion;
}}
