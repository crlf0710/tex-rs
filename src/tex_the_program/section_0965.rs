//! ` `

// @<Compute the trie op code, |v|...@>=
pub(crate) macro Compute_the_trie_op_code__v__and_set_l_to_0
    ($globals:expr, $k:expr, $l:expr, $v:expr) {{
        // if hc[1]=0 then hyf[0]:=0;
        if $globals.hc[1] == 0 {
            $globals.hyf[0] = 0.into();
        }
        // if hc[k]=0 then hyf[k]:=0;
        if $globals.hc[$k.get() as usize] == 0 {
            $globals.hyf[$k.get() as usize] = 0.into();
        }
        // l:=k; v:=min_quarterword;
        $l = $k;
        $v = min_quarterword;
        crate::region_forward_label!(
        |'done1|
        {
        // loop@+  begin if hyf[l]<>0 then v:=new_trie_op(k-l,hyf[l],v);
        loop {
            if $globals.hyf[$l.get() as usize] != 0 {
                $v = new_trie_op($globals, ($k - $l).get().into(), $globals.hyf[$l.get() as usize].get().into(), $v);
            }
            // if l>0 then decr(l)@+else goto done1;
            if $l > 0 {
                decr!($l);
            } else {
                crate::goto_forward_label!('done1);
            }
            // end;
        }
        }
        // done1:
        'done1 <-
        );

        use crate::section_0016::decr;
        use crate::section_0018::ASCII_code;
        use crate::section_0110::min_quarterword;
        use crate::section_0944::new_trie_op;
    }}
