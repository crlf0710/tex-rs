//! @ The \.{WEB} operation \.{@@\$} denotes the value that should be at the
//! end of this \.{TEX.POOL} file; any other value means that the wrong pool
//! file has been loaded.
//! @^check sum@>
//
// @<Check the pool check sum@>=
macro_rules! Check_the_pool_check_sum {
    ($globals:expr, $c:expr) => {
        // begin a:=0; k:=1;
        // loop@+  begin if (xord[n]<"0")or(xord[n]>"9") then
        //   bad_pool('! TEX.POOL check sum doesn''t have nine digits.');
        // @.TEX.POOL check sum...@>
        //   a:=10*a+xord[n]-"0";
        //   if k=9 then goto done;
        //   incr(k); read(pool_file,n);
        //   end;
        // done: if a<>@$ then bad_pool('! TEX.POOL doesn''t match; TANGLE me again.');
        // @.TEX.POOL doesn't match@>
        // c:=true;
        $c = true;
        // end
    }
}