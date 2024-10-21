/// The structure can be converted to LaTeX.
pub trait ToTex {
    /// Convert the object into LaTeX, consuming it.
    fn to_tex(self) -> String;
}
