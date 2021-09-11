//! ` `
// @<Set the value of |output_penalty|@>=
pub(crate) macro Set_the_value_of_output_penalty($globals:expr) {{
    // if type(best_page_break)=penalty_node then
    if r#type!($globals, $globals.best_page_break) == penalty_node {
        // begin geq_word_define(int_base+output_penalty_code,penalty(best_page_break));
        geq_word_define(
            $globals,
            int_base as pointer + output_penalty_code as pointer,
            penalty!($globals, $globals.best_page_break),
        );
        // penalty(best_page_break):=inf_penalty;
        penalty!($globals, $globals.best_page_break) = inf_penalty;
        // end
    }
    // else geq_word_define(int_base+output_penalty_code,inf_penalty)
    else {
        geq_word_define(
            $globals,
            int_base as pointer + output_penalty_code as pointer,
            inf_penalty,
        );
    }
    use crate::section_0115::pointer;
    use crate::section_0133::r#type;
    use crate::section_0157::inf_penalty;
    use crate::section_0157::penalty;
    use crate::section_0157::penalty_node;
    use crate::section_0230::int_base;
    use crate::section_0236::output_penalty_code;
    use crate::section_0279::geq_word_define;
}}
