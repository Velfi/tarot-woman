#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaceValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Page,
    Knight,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Cups,
    Pentacles,
    Wands,
    Swords,
}
