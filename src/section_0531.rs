// @ @<Scan file name in the buffer@>=
macro_rules! Scan_file_name_in_the_buffer {
    ($globals:expr) => {
        // begin begin_name; k:=first;
        // while (buffer[k]=" ")and(k<last) do incr(k);
        // loop@+  begin if k=last then goto done;
        //   if not more_name(buffer[k]) then goto done;
        //   incr(k);
        //   end;
        // done:end_name;
        // end
    };
}
