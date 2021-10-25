//! APIs for configurations.

/// Additionals APIs for configuring TeX interaction.
pub trait TeXConfiguration {
    /// Configure `error_line` parameter
    fn set_error_line(&mut self, error_line: u8);
    /// Configure `half_error_line` parameter
    fn set_half_error_line(&mut self, half_error_line: u8);
    /// Configure `max_print_line` parameter
    fn set_max_print_line(&mut self, max_print_line: u8);

    #[cfg(feature = "latex_support")]
    /// Configure `latex_support` parameter
    fn set_latex_support(&mut self, latex_support_enabled: bool);
}

impl TeXConfiguration for crate::section_0004::TeXGlobals {
    fn set_error_line(&mut self, error_line: u8) {
        self.error_line = error_line;
    }

    fn set_half_error_line(&mut self, error_line: u8) {
        self.half_error_line = error_line;
    }

    fn set_max_print_line(&mut self, max_print_line: u8) {
        self.max_print_line = max_print_line;
    }

    #[cfg(feature = "latex_support")]
    fn set_latex_support(&mut self, latex_support_enabled: bool) {
        self.latex_support_enabled = latex_support_enabled;
    }
}
