//! ` `
// @<Read the {\.{TFM}} size fields@>=
macro_rules! Read_the_TFM_size_fields {
    ($globals:expr) => {{
        todo!("read size fields");
        // begin read_sixteen(lf);
        // fget; read_sixteen(lh);
        // fget; read_sixteen(bc);
        // fget; read_sixteen(ec);
        // if (bc>ec+1)or(ec>255) then abort;
        // if bc>255 then {|bc=256| and |ec=255|}
        //   begin bc:=1; ec:=0;
        //   end;
        // fget; read_sixteen(nw);
        // fget; read_sixteen(nh);
        // fget; read_sixteen(nd);
        // fget; read_sixteen(ni);
        // fget; read_sixteen(nl);
        // fget; read_sixteen(nk);
        // fget; read_sixteen(ne);
        // fget; read_sixteen(np);
        // if lf<>6+lh+(ec-bc+1)+nw+nh+nd+ni+nl+nk+ne+np then abort;
        // if (nw=0)or(nh=0)or(nd=0)or(ni=0) then abort;
        // end
        //
    }}
}
