//! ` `

// @<Replace |z|...@>=
pub(crate) macro Replace_z_by_z_prime_and_compute_alpha_beta($globals:expr, $z:expr, $alpha:expr, $beta:expr) {{
    // begin alpha:=16;
    $alpha = 16;
    // while z>=@'40000000 do
    while $z.inner() >= 0o40000000 {
        // begin z:=z div 2; alpha:=alpha+alpha;
        $z = scaled::new_from_inner($z.inner() / 2);
        $alpha = $alpha + $alpha;
        // end;
    }
    // beta:=256 div alpha; alpha:=alpha*z;
    $beta = (256 / $alpha) as u8;
    $alpha = $alpha * $z.inner();
    // end
    use crate::section_0101::scaled;
}}
