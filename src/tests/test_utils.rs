#[cfg(test)]

pub mod shared {

    use crate::domain::voci::Lang;

    /// Constants

   pub const WORD: &str = "chien";
   pub const WORD_LANG: Lang = Lang::fr;
   pub const TRANSLATIONS: [&str; 2] = ["hund", "köter"];
   pub const TRANSLATION_LANG: Lang = Lang::de;
}
