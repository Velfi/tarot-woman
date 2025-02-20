use std::fmt::Formatter;

pub mod major_arcana;
pub mod minor_arcana;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    inner: Inner,
}

impl Card {
    pub fn wiki_image_link(&self) -> String {
        match self.inner {
            Inner::MajorArcana(major_arcana::Card::TheFool) => "https://en.wikipedia.org/wiki/The_Fool_(tarot_card)#/media/File:RWS_Tarot_00_Fool.jpg",
            Inner::MajorArcana(major_arcana::Card::TheMagician) => "https://en.wikipedia.org/wiki/The_Magician_(tarot_card)#/media/File:RWS_Tarot_01_Magician.jpg",
            Inner::MajorArcana(major_arcana::Card::TheHighPriestess) => "https://en.wikipedia.org/wiki/The_High_Priestess#/media/File:RWS_Tarot_02_High_Priestess.jpg",
            Inner::MajorArcana(major_arcana::Card::TheEmpress) => "https://en.wikipedia.org/wiki/The_Empress_(tarot_card)#/media/File:RWS_Tarot_03_Empress.jpg",
            Inner::MajorArcana(major_arcana::Card::TheEmperor) => "https://en.wikipedia.org/wiki/The_Emperor_(tarot_card)#/media/File:RWS_Tarot_04_Emperor.jpg",
            Inner::MajorArcana(major_arcana::Card::TheHierophant) => "https://en.wikipedia.org/wiki/The_Hierophant#/media/File:RWS_Tarot_05_Hierophant.jpg",
            Inner::MajorArcana(major_arcana::Card::TheLovers) => "https://en.wikipedia.org/wiki/The_Lovers_(tarot_card)#/media/File:RWS_Tarot_06_Lovers.jpg",
            Inner::MajorArcana(major_arcana::Card::TheChariot) => "https://en.wikipedia.org/wiki/The_Chariot_(tarot_card)#/media/File:RWS_Tarot_07_Chariot.jpg",
            Inner::MajorArcana(major_arcana::Card::Strength) => "https://en.wikipedia.org/wiki/Strength_(tarot_card)#/media/File:RWS_Tarot_08_Strength.jpg",
            Inner::MajorArcana(major_arcana::Card::TheHermit) => "https://en.wikipedia.org/wiki/The_Hermit#/media/File:RWS_Tarot_09_Hermit.jpg",
            Inner::MajorArcana(major_arcana::Card::WheelOfFortune) => "https://en.wikipedia.org/wiki/Wheel_of_Fortune_(tarot_card)#/media/File:RWS_Tarot_10_Wheel_of_Fortune.jpg",
            Inner::MajorArcana(major_arcana::Card::Justice) => "https://en.wikipedia.org/wiki/Justice_(tarot_card)#/media/File:RWS_Tarot_11_Justice.jpg",
            Inner::MajorArcana(major_arcana::Card::TheHangedMan) => "https://en.wikipedia.org/wiki/The_Hanged_Man_(tarot_card)#/media/File:RWS_Tarot_12_Hanged_Man.jpg",
            Inner::MajorArcana(major_arcana::Card::Death) => "https://en.wikipedia.org/wiki/Death_(tarot_card)#/media/File:RWS_Tarot_13_Death.jpg",
            Inner::MajorArcana(major_arcana::Card::Temperance) => "https://en.wikipedia.org/wiki/Temperance_(tarot_card)#/media/File:RWS_Tarot_14_Temperance.jpg",
            Inner::MajorArcana(major_arcana::Card::TheDevil) => "https://en.wikipedia.org/wiki/The_Devil_(tarot_card)#/media/File:RWS_Tarot_15_Devil.jpg",
            Inner::MajorArcana(major_arcana::Card::TheTower) => "https://en.wikipedia.org/wiki/The_Tower_(tarot_card)#/media/File:RWS_Tarot_16_Tower.jpg",
            Inner::MajorArcana(major_arcana::Card::TheStar) => "https://en.wikipedia.org/wiki/The_Star_(tarot_card)#/media/File:RWS_Tarot_17_Star.jpg",
            Inner::MajorArcana(major_arcana::Card::TheMoon) => "https://en.wikipedia.org/wiki/The_Moon_(tarot_card)#/media/File:RWS_Tarot_18_Moon.jpg",
            Inner::MajorArcana(major_arcana::Card::TheSun) => "https://en.wikipedia.org/wiki/The_Sun_(tarot_card)#/media/File:RWS_Tarot_19_Sun.jpg",
            Inner::MajorArcana(major_arcana::Card::Judgement) => "https://en.wikipedia.org/wiki/Judgement_(tarot_card)#/media/File:RWS_Tarot_20_Judgement.jpg",
            Inner::MajorArcana(major_arcana::Card::TheWorld) => "https://en.wikipedia.org/wiki/The_World_(tarot_card)#/media/File:RWS_Tarot_21_World.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Ace) => "https://en.wikipedia.org/wiki/Ace_of_Cups#/media/File:Cups01.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Two) => "https://en.wikipedia.org/wiki/Two_of_Cups#/media/File:Cups02.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Three) => "https://en.wikipedia.org/wiki/Three_of_Cups#/media/File:Cups03.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Four) => "https://en.wikipedia.org/wiki/Four_of_Cups#/media/File:Cups04.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Five) => "https://en.wikipedia.org/wiki/Five_of_Cups#/media/File:Cups05.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Six) => "https://en.wikipedia.org/wiki/Six_of_Cups#/media/File:Cups06.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Seven) => "https://en.wikipedia.org/wiki/Seven_of_Cups#/media/File:Cups07.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Eight) => "https://en.wikipedia.org/wiki/Eight_of_Cups#/media/File:Cups08.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Nine) => "https://en.wikipedia.org/wiki/Nine_of_Cups#/media/File:Cups09.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Ten) => "https://en.wikipedia.org/wiki/Ten_of_Cups#/media/File:Cups10.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Page) => "https://en.wikipedia.org/wiki/Page_of_Cups#/media/File:Cups11.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Knight) => "https://en.wikipedia.org/wiki/Knight_of_Cups#/media/File:Cups12.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Queen) => "https://en.wikipedia.org/wiki/Queen_of_Cups#/media/File:Cups13.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::King) => "https://en.wikipedia.org/wiki/King_of_Cups#/media/File:Cups14.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Ace) => "https://en.wikipedia.org/wiki/Ace_of_Wands#/media/File:Wands01.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Two) => "https://en.wikipedia.org/wiki/Two_of_Wands#/media/File:Wands02.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Three) => "https://en.wikipedia.org/wiki/Three_of_Wands#/media/File:Wands03.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Four) => "https://en.wikipedia.org/wiki/Four_of_Wands#/media/File:Wands04.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Five) => "https://en.wikipedia.org/wiki/Five_of_Wands#/media/File:Wands05.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Six) => "https://en.wikipedia.org/wiki/Six_of_Wands#/media/File:Wands06.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Seven) => "https://en.wikipedia.org/wiki/Seven_of_Wands#/media/File:Wands07.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Eight) => "https://en.wikipedia.org/wiki/Eight_of_Wands#/media/File:Wands08.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Nine) => "https://en.wikipedia.org/wiki/Nine_of_Wands#/media/File:Wands09.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Ten) => "https://en.wikipedia.org/wiki/Ten_of_Wands#/media/File:Wands10.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Page) => "https://en.wikipedia.org/wiki/Page_of_Wands#/media/File:Wands11.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Knight) => "https://en.wikipedia.org/wiki/Knight_of_Wands#/media/File:Wands12.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Queen) => "https://en.wikipedia.org/wiki/Queen_of_Wands#/media/File:Wands13.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::King) => "https://en.wikipedia.org/wiki/King_of_Wands#/media/File:Wands14.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Ace) => "https://en.wikipedia.org/wiki/Ace_of_Swords#/media/File:Swords01.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Two) => "https://en.wikipedia.org/wiki/Two_of_Swords#/media/File:Swords02.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Three) => "https://en.wikipedia.org/wiki/Three_of_Swords#/media/File:Swords03.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Four) => "https://en.wikipedia.org/wiki/Four_of_Swords#/media/File:Swords04.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Five) => "https://en.wikipedia.org/wiki/Five_of_Swords#/media/File:Swords05.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Six) => "https://en.wikipedia.org/wiki/Six_of_Swords#/media/File:Swords06.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Seven) => "https://en.wikipedia.org/wiki/Seven_of_Swords#/media/File:Swords07.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Eight) => "https://en.wikipedia.org/wiki/Eight_of_Swords#/media/File:Swords08.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Nine) => "https://en.wikipedia.org/wiki/Nine_of_Swords#/media/File:Swords09.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Ten) => "https://en.wikipedia.org/wiki/Ten_of_Swords#/media/File:Swords10.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Page) => "https://en.wikipedia.org/wiki/Page_of_Swords#/media/File:Swords11.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Knight) => "https://en.wikipedia.org/wiki/Knight_of_Swords#/media/File:Swords12.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Queen) => "https://en.wikipedia.org/wiki/Queen_of_Swords#/media/File:Swords13.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::King) => "https://en.wikipedia.org/wiki/King_of_Swords#/media/File:Swords14.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Ace) => "https://en.wikipedia.org/wiki/Ace_of_Pentacles#/media/File:Pents01.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Two) => "https://en.wikipedia.org/wiki/Two_of_Pentacles#/media/File:Pents02.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Three) => "https://en.wikipedia.org/wiki/Three_of_Pentacles#/media/File:Pents03.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Four) => "https://en.wikipedia.org/wiki/Four_of_Pentacles#/media/File:Pents04.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Five) => "https://en.wikipedia.org/wiki/Five_of_Pentacles#/media/File:Pents05.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Six) => "https://en.wikipedia.org/wiki/Six_of_Pentacles#/media/File:Pents06.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Seven) => "https://en.wikipedia.org/wiki/Seven_of_Pentacles#/media/File:Pents07.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Eight) => "https://en.wikipedia.org/wiki/Eight_of_Pentacles#/media/File:Pents08.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Nine) => "https://en.wikipedia.org/wiki/Nine_of_Pentacles#/media/File:Pents09.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Ten) => "https://en.wikipedia.org/wiki/Ten_of_Pentacles#/media/File:Pents10.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Page) => "https://en.wikipedia.org/wiki/Page_of_Pentacles#/media/File:Pents11.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Knight) => "https://en.wikipedia.org/wiki/Knight_of_Pentacles#/media/File:Pents12.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Queen) => "https://en.wikipedia.org/wiki/Queen_of_Pentacles#/media/File:Pents13.jpg",
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::King) => "https://en.wikipedia.org/wiki/King_of_Pentacles#/media/File:Pents14.jpg",
        }.to_string()
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.inner {
            Inner::MajorArcana(major_arcana::Card::TheFool) => write!(f, "The Fool"),
            Inner::MajorArcana(major_arcana::Card::TheMagician) => write!(f, "The Magician"),
            Inner::MajorArcana(major_arcana::Card::TheHighPriestess) => {
                write!(f, "The High Priestess")
            }
            Inner::MajorArcana(major_arcana::Card::TheEmpress) => write!(f, "The Empress"),
            Inner::MajorArcana(major_arcana::Card::TheEmperor) => write!(f, "The Emperor"),
            Inner::MajorArcana(major_arcana::Card::TheHierophant) => write!(f, "The Hierophant"),
            Inner::MajorArcana(major_arcana::Card::TheLovers) => write!(f, "The Lovers"),
            Inner::MajorArcana(major_arcana::Card::TheChariot) => write!(f, "The Chariot"),
            Inner::MajorArcana(major_arcana::Card::Strength) => write!(f, "Strength"),
            Inner::MajorArcana(major_arcana::Card::TheHermit) => write!(f, "The Hermit"),
            Inner::MajorArcana(major_arcana::Card::WheelOfFortune) => write!(f, "Wheel of Fortune"),
            Inner::MajorArcana(major_arcana::Card::Justice) => write!(f, "Justice"),
            Inner::MajorArcana(major_arcana::Card::TheHangedMan) => write!(f, "The Hanged Man"),
            Inner::MajorArcana(major_arcana::Card::Death) => write!(f, "Death"),
            Inner::MajorArcana(major_arcana::Card::Temperance) => write!(f, "Temperance"),
            Inner::MajorArcana(major_arcana::Card::TheDevil) => write!(f, "The Devil"),
            Inner::MajorArcana(major_arcana::Card::TheTower) => write!(f, "The Tower"),
            Inner::MajorArcana(major_arcana::Card::TheStar) => write!(f, "The Star"),
            Inner::MajorArcana(major_arcana::Card::TheMoon) => write!(f, "The Moon"),
            Inner::MajorArcana(major_arcana::Card::TheSun) => write!(f, "The Sun"),
            Inner::MajorArcana(major_arcana::Card::Judgement) => write!(f, "Judgement"),
            Inner::MajorArcana(major_arcana::Card::TheWorld) => write!(f, "The World"),
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Ace) => {
                write!(f, "Ace of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Two) => {
                write!(f, "Two of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Three) => {
                write!(f, "Three of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Four) => {
                write!(f, "Four of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Five) => {
                write!(f, "Five of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Six) => {
                write!(f, "Six of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Seven) => {
                write!(f, "Seven of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Eight) => {
                write!(f, "Eight of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Nine) => {
                write!(f, "Nine of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Ten) => {
                write!(f, "Ten of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Page) => {
                write!(f, "Page of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Knight) => {
                write!(f, "Knight of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Queen) => {
                write!(f, "Queen of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::King) => {
                write!(f, "King of Cups")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Ace) => {
                write!(f, "Ace of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Two) => {
                write!(f, "Two of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Three) => {
                write!(f, "Three of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Four) => {
                write!(f, "Four of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Five) => {
                write!(f, "Five of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Six) => {
                write!(f, "Six of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Seven) => {
                write!(f, "Seven of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Eight) => {
                write!(f, "Eight of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Nine) => {
                write!(f, "Nine of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Ten) => {
                write!(f, "Ten of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Page) => {
                write!(f, "Page of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Knight) => {
                write!(f, "Knight of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Queen) => {
                write!(f, "Queen of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::King) => {
                write!(f, "King of Wands")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Ace) => {
                write!(f, "Ace of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Two) => {
                write!(f, "Two of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Three) => {
                write!(f, "Three of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Four) => {
                write!(f, "Four of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Five) => {
                write!(f, "Five of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Six) => {
                write!(f, "Six of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Seven) => {
                write!(f, "Seven of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Eight) => {
                write!(f, "Eight of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Nine) => {
                write!(f, "Nine of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Ten) => {
                write!(f, "Ten of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Page) => {
                write!(f, "Page of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Knight) => {
                write!(f, "Knight of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Queen) => {
                write!(f, "Queen of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::King) => {
                write!(f, "King of Swords")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Ace) => {
                write!(f, "Ace of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Two) => {
                write!(f, "Two of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Three) => {
                write!(f, "Three of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Four) => {
                write!(f, "Four of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Five) => {
                write!(f, "Five of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Six) => {
                write!(f, "Six of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Seven) => {
                write!(f, "Seven of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Eight) => {
                write!(f, "Eight of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Nine) => {
                write!(f, "Nine of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Ten) => {
                write!(f, "Ten of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Page) => {
                write!(f, "Page of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Knight) => {
                write!(f, "Knight of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Queen) => {
                write!(f, "Queen of Pentacles")
            }
            Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::King) => {
                write!(f, "King of Pentacles")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Inner {
    MajorArcana(major_arcana::Card),
    MinorArcana(minor_arcana::Suit, minor_arcana::FaceValue),
}

pub const MAJOR_ARCANA: &[Card] = &[
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheFool),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheMagician),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheHighPriestess),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheEmpress),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheEmperor),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheHierophant),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheLovers),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheChariot),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::Strength),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheHermit),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::WheelOfFortune),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::Justice),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheHangedMan),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::Death),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::Temperance),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheDevil),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheTower),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheStar),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheMoon),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheSun),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::Judgement),
    },
    Card {
        inner: Inner::MajorArcana(major_arcana::Card::TheWorld),
    },
];

pub const MINOR_ARCANA: &[Card] = &[
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Ace),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Two),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Three),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Four),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Five),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Six),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Seven),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Eight),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Nine),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Ten),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Page),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Knight),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::Queen),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Cups, minor_arcana::FaceValue::King),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Ace),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Two),
    },
    Card {
        inner: Inner::MinorArcana(
            minor_arcana::Suit::Pentacles,
            minor_arcana::FaceValue::Three,
        ),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Four),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Five),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Six),
    },
    Card {
        inner: Inner::MinorArcana(
            minor_arcana::Suit::Pentacles,
            minor_arcana::FaceValue::Seven,
        ),
    },
    Card {
        inner: Inner::MinorArcana(
            minor_arcana::Suit::Pentacles,
            minor_arcana::FaceValue::Eight,
        ),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Nine),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Ten),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::Page),
    },
    Card {
        inner: Inner::MinorArcana(
            minor_arcana::Suit::Pentacles,
            minor_arcana::FaceValue::Knight,
        ),
    },
    Card {
        inner: Inner::MinorArcana(
            minor_arcana::Suit::Pentacles,
            minor_arcana::FaceValue::Queen,
        ),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Pentacles, minor_arcana::FaceValue::King),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Ace),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Two),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Three),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Four),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Five),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Six),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Seven),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Eight),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Nine),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Ten),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Page),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Knight),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::Queen),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Wands, minor_arcana::FaceValue::King),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Ace),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Two),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Three),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Four),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Five),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Six),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Seven),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Eight),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Nine),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Ten),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Page),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Knight),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::Queen),
    },
    Card {
        inner: Inner::MinorArcana(minor_arcana::Suit::Swords, minor_arcana::FaceValue::King),
    },
];
