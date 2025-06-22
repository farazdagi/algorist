use {
    crate::io::Scanner,
    std::{fmt, io::BufRead},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CardRank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for CardRank {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

impl From<CardRank> for char {
    fn from(c: CardRank) -> Self {
        match c {
            CardRank::Two => '2',
            CardRank::Three => '3',
            CardRank::Four => '4',
            CardRank::Five => '5',
            CardRank::Six => '6',
            CardRank::Seven => '7',
            CardRank::Eight => '8',
            CardRank::Nine => '9',
            CardRank::Ten => 'T',
            CardRank::Jack => 'J',
            CardRank::Queen => 'Q',
            CardRank::King => 'K',
            CardRank::Ace => 'A',
        }
    }
}

impl std::fmt::Debug for CardRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as Into<char>>::into(*self))
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CardSuit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl CardSuit {
    pub fn all() -> impl Iterator<Item = Self> {
        [Self::Clubs, Self::Diamonds, Self::Hearts, Self::Spades].into_iter()
    }

    pub fn filter<F>(f: F) -> impl Iterator<Item = Self>
    where
        F: Fn(Self) -> bool,
    {
        Self::all().filter(move |&suit| f(suit))
    }
}

impl From<char> for CardSuit {
    fn from(c: char) -> Self {
        match c {
            'C' => Self::Clubs,
            'D' => Self::Diamonds,
            'H' => Self::Hearts,
            'S' => Self::Spades,
            _ => unreachable!(),
        }
    }
}

impl From<CardSuit> for char {
    fn from(c: CardSuit) -> Self {
        match c {
            CardSuit::Clubs => 'C',
            CardSuit::Diamonds => 'D',
            CardSuit::Hearts => 'H',
            CardSuit::Spades => 'S',
        }
    }
}

impl std::fmt::Debug for CardSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suit = match self {
            Self::Clubs => "♣",
            Self::Diamonds => "♢",
            Self::Hearts => "♡",
            Self::Spades => "♠",
        };
        write!(f, "{suit}")
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Card(CardRank, CardSuit);

impl Card {
    pub fn new(rank: CardRank, suit: CardSuit) -> Self {
        Self(rank, suit)
    }

    pub fn rank(&self) -> CardRank {
        self.0
    }

    pub fn suit(&self) -> CardSuit {
        self.1
    }

    pub fn is_trump(&self, trump: CardSuit) -> bool {
        self.1 == trump
    }

    pub fn is_same_suit(&self, other: &Self) -> bool {
        self.1 == other.1
    }

    pub fn is_same_rank(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = <CardRank as Into<char>>::into(self.0);
        let suit = <CardSuit as Into<char>>::into(self.1);
        write!(f, "{rank}{suit}")
    }
}

impl From<String> for Card {
    fn from(s: String) -> Self {
        assert!(s.len() == 2);
        let s = s.chars().collect::<Vec<_>>();
        Self::new(CardRank::from(s[0]), CardSuit::from(s[1]))
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{:?}", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct CardDeck {
    cards: Vec<Card>,
    trump: Option<CardSuit>,
    by_suit: Vec<Vec<Card>>,
}

impl Default for CardDeck {
    fn default() -> Self {
        Self::new()
    }
}

impl CardDeck {
    pub fn new() -> Self {
        let mut cards = vec![];
        let mut by_suit = vec![vec![]; 4];
        for &suit in &[
            CardSuit::Clubs,
            CardSuit::Diamonds,
            CardSuit::Hearts,
            CardSuit::Spades,
        ] {
            for &rank in &[
                CardRank::Two,
                CardRank::Three,
                CardRank::Four,
                CardRank::Five,
                CardRank::Six,
                CardRank::Seven,
                CardRank::Eight,
                CardRank::Nine,
                CardRank::Ten,
                CardRank::Jack,
                CardRank::Queen,
                CardRank::King,
                CardRank::Ace,
            ] {
                cards.push(Card::new(rank, suit));
                by_suit[suit as usize].push(Card::new(rank, suit));
            }
        }
        Self {
            cards,
            trump: None,
            by_suit,
        }
    }

    pub fn from_scan<B: BufRead>(scan: &mut Scanner<B>, n: usize, trump: Option<CardSuit>) -> Self {
        let mut cards = vec![];
        let mut by_suit = vec![vec![]; 4];
        for _ in 0..n {
            let card = Card::from(scan.string());
            cards.push(card);
            by_suit[card.suit() as usize].push(card);
        }
        Self {
            cards,
            trump,
            by_suit,
        }
    }

    pub fn from_vec(cards: Vec<Card>, trump: Option<CardSuit>) -> Self {
        let mut by_suit = vec![vec![]; 4];
        for card in &cards {
            by_suit[card.suit() as usize].push(*card);
        }
        Self {
            cards,
            trump,
            by_suit,
        }
    }

    #[must_use]
    pub fn sorted(self) -> Self {
        let mut cards = self.cards;
        cards.sort();
        let mut by_suit = self.by_suit;
        for suit in &mut by_suit {
            suit.sort();
        }

        Self {
            cards,
            trump: self.trump,
            by_suit,
        }
    }

    pub fn set_trump(&mut self, trump: CardSuit) {
        self.trump = Some(trump);
    }

    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    pub fn cards_by_suit(&self, suit: CardSuit) -> &[Card] {
        &self.by_suit[suit as usize]
    }

    pub fn trump(&self) -> Option<CardSuit> {
        self.trump
    }

    pub fn is_trump(&self, suit: CardSuit) -> bool {
        self.trump == Some(suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_conversion() {
        let s = "2H";
        let card = Card::from(s.to_string());
        assert_eq!(card.rank(), CardRank::Two);
        assert_eq!(card.suit(), CardSuit::Hearts);

        let s = "AS";
        let card = Card::from(s.to_string());
        assert_eq!(card.rank(), CardRank::Ace);
        assert_eq!(card.suit(), CardSuit::Spades);
    }

    #[test]
    fn all_variants() {
        for rank in 0..13 {
            for suit in 0..4 {
                let (rank, suit) = (
                    "23456789TJQKA".chars().nth(rank).unwrap(),
                    "CDHS".chars().nth(suit).unwrap(),
                );
                let s = format!("{}{}", rank, suit);
                let card = Card::from(s);
                assert_eq!(card.rank(), CardRank::from(rank));
                assert_eq!(card.suit(), CardSuit::from(suit));
            }
        }
    }

    #[test]
    fn sort_card_ranks() {
        let mut ranks = vec![
            CardRank::Ace,
            CardRank::Eight,
            CardRank::Two,
            CardRank::Four,
            CardRank::Five,
            CardRank::Six,
            CardRank::Seven,
            CardRank::Three,
            CardRank::Nine,
            CardRank::Ten,
            CardRank::Jack,
            CardRank::Queen,
            CardRank::King,
        ];
        ranks.sort();
        assert_eq!(ranks, vec![
            CardRank::Two,
            CardRank::Three,
            CardRank::Four,
            CardRank::Five,
            CardRank::Six,
            CardRank::Seven,
            CardRank::Eight,
            CardRank::Nine,
            CardRank::Ten,
            CardRank::Jack,
            CardRank::Queen,
            CardRank::King,
            CardRank::Ace
        ]);
    }

    #[test]
    fn sort_cards() {
        let mut cards = vec![
            Card::from("QH".to_string()),
            Card::from("9H".to_string()),
            Card::from("7H".to_string()),
            Card::from("2H".to_string()),
            Card::from("3H".to_string()),
            Card::from("4H".to_string()),
            Card::from("AH".to_string()),
            Card::from("5H".to_string()),
            Card::from("6H".to_string()),
            Card::from("8H".to_string()),
            Card::from("TH".to_string()),
            Card::from("JH".to_string()),
            Card::from("KH".to_string()),
        ];
        cards.sort();
        assert_eq!(cards, vec![
            Card::from("2H".to_string()),
            Card::from("3H".to_string()),
            Card::from("4H".to_string()),
            Card::from("5H".to_string()),
            Card::from("6H".to_string()),
            Card::from("7H".to_string()),
            Card::from("8H".to_string()),
            Card::from("9H".to_string()),
            Card::from("TH".to_string()),
            Card::from("JH".to_string()),
            Card::from("QH".to_string()),
            Card::from("KH".to_string()),
            Card::from("AH".to_string()),
        ]);
    }

    #[test]
    fn sord_cards_mixed_suits() {
        let mut cards = vec![
            Card::from("QS".to_string()),
            Card::from("QH".to_string()),
            Card::from("QD".to_string()),
            Card::from("9S".to_string()),
            Card::from("7H".to_string()),
            Card::from("2D".to_string()),
            Card::from("3C".to_string()),
            Card::from("4H".to_string()),
            Card::from("AS".to_string()),
            Card::from("5H".to_string()),
            Card::from("6H".to_string()),
            Card::from("8H".to_string()),
            Card::from("TS".to_string()),
            Card::from("JC".to_string()),
            Card::from("KD".to_string()),
        ];
        cards.sort();
        assert_eq!(cards, vec![
            Card::from("2D".to_string()),
            Card::from("3C".to_string()),
            Card::from("4H".to_string()),
            Card::from("5H".to_string()),
            Card::from("6H".to_string()),
            Card::from("7H".to_string()),
            Card::from("8H".to_string()),
            Card::from("9S".to_string()),
            Card::from("TS".to_string()),
            Card::from("JC".to_string()),
            Card::from("QS".to_string()),
            Card::from("QH".to_string()),
            Card::from("QD".to_string()),
            Card::from("KD".to_string()),
            Card::from("AS".to_string()),
        ]);
    }

    #[test]
    fn compare_cards() {
        let c1 = Card::from("QH".to_string());
        let c2 = Card::from("QS".to_string());
        assert!(c1 != c2);

        let c1 = Card::from("QH".to_string());
        let c2 = Card::from("QH".to_string());
        assert!(c1 == c2);

        let c1 = Card::from("QH".to_string());
        assert!(c1.is_trump(CardSuit::Hearts));

        let c1 = Card::from("2H".to_string());
        let c2 = Card::from("2S".to_string());
        assert!(c1.is_same_rank(&c2));

        let c1 = Card::from("QS".to_string());
        let c2 = Card::from("KS".to_string());
        assert!(c1.is_same_suit(&c2));
    }

    #[test]
    fn deck_creation() {
        let deck = CardDeck::new();
        assert_eq!(deck.cards().len(), 52);
        assert_eq!(deck.cards_by_suit(CardSuit::Clubs).len(), 13);
        assert_eq!(deck.cards_by_suit(CardSuit::Diamonds).len(), 13);
        assert_eq!(deck.cards_by_suit(CardSuit::Hearts).len(), 13);
        assert_eq!(deck.cards_by_suit(CardSuit::Spades).len(), 13);
    }

    #[test]
    fn deck_from_scan() {
        let input = b"2H 3H 4H 5H 6H\n";
        let mut scan = Scanner::new(std::io::BufReader::new(input.as_ref()));
        let mut deck = CardDeck::from_scan(&mut scan, 5, None);
        assert_eq!(deck.cards().len(), 5);
        assert_eq!(deck.cards_by_suit(CardSuit::Hearts).len(), 5);
        deck.set_trump(CardSuit::Hearts);
        assert_eq!(deck.trump(), Some(CardSuit::Hearts));
    }

    #[test]
    fn deck_from_scan_split_by_suit() {
        let input = b"AC 3H 4H 5H TS 6H 2H KS AD\n";
        let mut scan = Scanner::new(std::io::BufReader::new(input.as_ref()));
        let deck = CardDeck::from_scan(&mut scan, 9, None);
        assert_eq!(deck.cards_by_suit(CardSuit::Hearts).len(), 5);
        assert_eq!(deck.cards_by_suit(CardSuit::Clubs).len(), 1);
        assert_eq!(deck.cards_by_suit(CardSuit::Spades).len(), 2);
        assert_eq!(deck.cards_by_suit(CardSuit::Diamonds).len(), 1);

        let mut cards = deck.cards_by_suit(CardSuit::Hearts).to_vec();
        cards.sort();
        assert_eq!(cards, vec![
            Card::from("2H".to_string()),
            Card::from("3H".to_string()),
            Card::from("4H".to_string()),
            Card::from("5H".to_string()),
            Card::from("6H".to_string()),
        ]);

        let mut cards = deck.cards_by_suit(CardSuit::Clubs).to_vec();
        cards.sort();
        assert_eq!(cards, vec![Card::from("AC".to_string())]);

        let mut cards = deck.cards_by_suit(CardSuit::Spades).to_vec();
        cards.sort();
        assert_eq!(cards, vec![
            Card::from("TS".to_string()),
            Card::from("KS".to_string()),
        ]);

        let mut cards = deck.cards_by_suit(CardSuit::Diamonds).to_vec();
        cards.sort();
        assert_eq!(cards, vec![Card::from("AD".to_string())]);
    }

    #[test]
    fn from_vec() {
        let cards = vec![
            Card::from("AC".to_string()),
            Card::from("3H".to_string()),
            Card::from("4H".to_string()),
            Card::from("5H".to_string()),
            Card::from("TS".to_string()),
            Card::from("6H".to_string()),
            Card::from("2H".to_string()),
            Card::from("KS".to_string()),
            Card::from("AD".to_string()),
        ];
        let deck = CardDeck::from_vec(cards, None);

        assert_eq!(deck.cards_by_suit(CardSuit::Hearts).len(), 5);
        assert_eq!(deck.cards_by_suit(CardSuit::Clubs).len(), 1);
        assert_eq!(deck.cards_by_suit(CardSuit::Spades).len(), 2);
        assert_eq!(deck.cards_by_suit(CardSuit::Diamonds).len(), 1);
    }

    #[test]
    fn deck_sorted() {
        let cards = vec![
            Card::from("AC".to_string()),
            Card::from("3H".to_string()),
            Card::from("4H".to_string()),
            Card::from("5H".to_string()),
            Card::from("TS".to_string()),
            Card::from("6H".to_string()),
            Card::from("2H".to_string()),
            Card::from("KS".to_string()),
            Card::from("AD".to_string()),
        ];
        let deck = CardDeck::from_vec(cards, None).sorted();
        let mut cards = deck.cards_by_suit(CardSuit::Hearts).to_vec();
        cards.sort();
        assert_eq!(cards, vec![
            Card::from("2H".to_string()),
            Card::from("3H".to_string()),
            Card::from("4H".to_string()),
            Card::from("5H".to_string()),
            Card::from("6H".to_string()),
        ]);

        let deck_sorted = deck.sorted();
        assert_eq!(cards, deck_sorted.cards_by_suit(CardSuit::Hearts));
    }

    #[test]
    fn card_suits() {
        assert_eq!(CardSuit::all().collect::<Vec<_>>(), vec![
            CardSuit::Clubs,
            CardSuit::Diamonds,
            CardSuit::Hearts,
            CardSuit::Spades,
        ]);

        assert_eq!(
            CardSuit::filter(|suit| suit != CardSuit::Clubs).collect::<Vec<_>>(),
            vec![CardSuit::Diamonds, CardSuit::Hearts, CardSuit::Spades]
        );
    }
}
