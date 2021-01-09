//! ` `
// @<Compute the sum of two glue specs@>=
// begin q:=new_spec(cur_val); r:=equiv(l);
// delete_glue_ref(cur_val);
// width(q):=width(q)+width(r);
// if stretch(q)=0 then stretch_order(q):=normal;
// if stretch_order(q)=stretch_order(r) then stretch(q):=stretch(q)+stretch(r)
// else if (stretch_order(q)<stretch_order(r))and(stretch(r)<>0) then
//   begin stretch(q):=stretch(r); stretch_order(q):=stretch_order(r);
//   end;
// if shrink(q)=0 then shrink_order(q):=normal;
// if shrink_order(q)=shrink_order(r) then shrink(q):=shrink(q)+shrink(r)
// else if (shrink_order(q)<shrink_order(r))and(shrink(r)<>0) then
//   begin shrink(q):=shrink(r); shrink_order(q):=shrink_order(r);
//   end;
// cur_val:=q;
// end
//
