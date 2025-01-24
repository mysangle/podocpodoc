#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AnnotationType {
    LineNumber,
    Match,
    SelectedMatch,
    Number,
    Keyword,
    Type,
    KnownValue,
    Char,
    LifetimeSpecifier,
    Comment,
    String
}
