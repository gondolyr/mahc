pub mod calc;
pub(crate) mod hand;
pub mod yaku;

/// Characters that represent terminal or honor tiles.
// Using a fixed array gets stored on the stack rather than a `String` which gets stored on the heap.
const TERMINAL_CHARS: [char; 9] = ['1', '9', 'E', 'S', 'W', 'N', 'r', 'g', 'w'];

#[derive(Debug)]
pub enum LimitHands {
    Mangan,
    Haneman,
    Baiman,
    Sanbaiman,
    KazoeYakuman,
}

#[derive(Debug, PartialEq)]
pub enum HandErr {
    InvalidGroup,
    InvalidSuit,
    InvalidShape,
    NoYaku,
    NoHandTiles,
    NoWinTile,
    DuplicateRiichi,
    IppatsuWithoutRiichi,
    DoubleRiichiHaiteiIppatsu,
    DoubleRiichiHaiteiChankan,
    ChankanTsumo,
    RinshanKanWithoutKan,
    RinshanWithoutTsumo,
    RinshanIppatsu,
    NoHan,
    NoFu,
}

impl std::fmt::Display for HandErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidGroup => write!(f, "Invalid Group found"),
            Self::InvalidSuit => write!(f, "Invalid Suit found"),
            Self::InvalidShape => write!(f, "Invalid Hand Shape found"),
            Self::NoYaku => write!(f, "No Yaku"),
            Self::NoHandTiles => write!(f, "No Hand Tiles given"),
            Self::NoWinTile => write!(f, "No Win Tile given"),
            Self::DuplicateRiichi => write!(f, "Cant Riichi and Double Riichi Simultaneously"),
            Self::IppatsuWithoutRiichi => write!(f, "Cant Ippatsu without Riichi"),
            Self::ChankanTsumo => write!(f, "Cant Tsumo and Chankan"),
            Self::RinshanKanWithoutKan => write!(f, "Cant Rinshan without Kan"),
            Self::RinshanWithoutTsumo => write!(f, "Cant Rinshan without Tsumo"),
            Self::RinshanIppatsu => write!(f, "Cant Rinshan and Ippatsu"),
            Self::DoubleRiichiHaiteiIppatsu => {
                write!(f, "Cant Double Riichi, Ippatsu and Haitei")
            }
            Self::DoubleRiichiHaiteiChankan => {
                write!(f, "Cant Double Riichi, Ippatsu and Haitei")
            }
            Self::NoHan => write!(f, "No Han provided!"),
            Self::NoFu => write!(f, "No Fu provided!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Fu {
    BasePoints,
    BasePointsChitoi,
    ClosedRon,
    Tsumo,
    NonSimpleClosedTriplet,
    SimpleClosedTriplet,
    NonSimpleOpenTriplet,
    SimpleOpenTriplet,
    NonSimpleClosedKan,
    SimpleClosedKan,
    NonSimpleOpenKan,
    SimpleOpenKan,
    Toitsu,
    SingleWait,
}

impl std::fmt::Display for Fu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BasePoints => write!(f, "BasePoints: 20"),
            Self::BasePointsChitoi => write!(f, "BasePoints: 25"),
            Self::ClosedRon => write!(f, "ClosedRon: 10"),
            Self::Tsumo => write!(f, "Tsumo: 2"),
            Self::NonSimpleClosedTriplet => write!(f, "NonSimpleClosedTriplet: 8"),
            Self::SimpleClosedTriplet => write!(f, "ClosedTriplet: 4"),
            Self::NonSimpleOpenTriplet => write!(f, "NonSimpleOpenTriplet: 4"),
            Self::SimpleOpenTriplet => write!(f, "OpenTriplet: 2"),
            Self::NonSimpleClosedKan => write!(f, "NonSimpleClosedKan: 32"),
            Self::SimpleClosedKan => write!(f, "ClosedKan: 16"),
            Self::NonSimpleOpenKan => write!(f, "NonSimpleOpenKan: 16"),
            Self::SimpleOpenKan => write!(f, "OpenKan: 8"),
            Self::Toitsu => write!(f, "Toitsu: 2"),
            Self::SingleWait => write!(f, "SingleWait: 2"),
        }
    }
}

impl Fu {
    /// Get the minipoint value.
    pub fn value(&self) -> u16 {
        match self {
            Self::BasePoints => 20,
            Self::BasePointsChitoi => 25,
            Self::ClosedRon => 10,
            Self::Tsumo => 2,
            Self::NonSimpleClosedTriplet => 8,
            Self::SimpleClosedTriplet => 4,
            Self::NonSimpleOpenTriplet => 4,
            Self::SimpleOpenTriplet => 2,
            Self::NonSimpleClosedKan => 32,
            Self::SimpleClosedKan => 16,
            Self::NonSimpleOpenKan => 16,
            Self::SimpleOpenKan => 8,
            Self::Toitsu => 2,
            Self::SingleWait => 2,
        }
    }
}

/// Sum up all of the fu, rounding to the nearest 10.
pub fn calculate_total_fu_value(fu: &[Fu]) -> u16 {
    ((fu.iter().map(|f| f.value()).sum::<u16>() + 9) / 10) * 10
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileGroup {
    pub value: String,
    pub suit: Suit,
    pub isopen: bool,
    pub group_type: GroupType,
    pub isterminal: bool,
}

impl TileGroup {
    pub fn new(group: String) -> Result<Self, HandErr> {
        let isopen = group.chars().last().unwrap().to_string() == "o";
        let value = group.chars().nth(0).unwrap().to_string();

        let suit = if !isopen {
            group.chars().last().unwrap().to_string()
        } else {
            group.chars().nth(group.len() - 2).unwrap().to_string()
        };
        let suit = Suit::suit_from_string(suit)?;

        let group_type = GroupType::group_type_from_string(group.to_string())?;

        let mut isterminal = false;
        if group_type == GroupType::Sequence {
            if value == "1" || value == "7" {
                isterminal = true;
            }
        } else if value == "1" || value == "9" {
            isterminal = true;
        }

        let tile = Self {
            value,
            suit,
            isopen,
            group_type,
            isterminal,
        };

        Ok(tile)
    }

    /// Check if the group is an honor.
    pub fn is_honor(&self) -> bool {
        matches!(self.suit, Suit::Wind | Suit::Dragon)
    }

    /// Parse the group value into an integer.
    pub fn into_u8(&self) -> Result<u8, std::num::ParseIntError> {
        self.value.parse()
    }
}

//AHAHAHAHAHAHAHAH I DONT NEED THIS
//turns our i did need this :)
#[derive(Debug, Clone, PartialEq)]
pub enum GroupType {
    Sequence,
    Triplet,
    Kan,
    Pair,
    None,
}

impl GroupType {
    /// Parse the group type from the string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mahc::GroupType;
    ///
    /// let input = "789s".to_string();
    /// let actual = GroupType::group_type_from_string(input);
    /// let expected = Ok(GroupType::Sequence);
    ///
    /// assert_eq!(actual, expected);
    /// ```
    pub fn group_type_from_string(group: String) -> Result<Self, HandErr> {
        let count = if group.contains('o') {
            group.len() - 2
        } else {
            group.len() - 1
        };

        if let Some(sub_group) = group.get(0..count) {
            for i in sub_group.chars() {
                if !"123456789ESWNrgw".contains(i) {
                    return Err(HandErr::InvalidGroup);
                }
            }
        } else {
            return Err(HandErr::InvalidGroup);
        }

        match count {
            2 => Ok(Self::Pair),
            3 => {
                if group.chars().nth(0).unwrap() == group.chars().nth(1).unwrap()
                    && group.chars().nth(1).unwrap() == group.chars().nth(2).unwrap()
                {
                    Ok(Self::Triplet)
                } else if ["123", "234", "345", "456", "567", "678", "789"]
                    .iter()
                    .cloned()
                    .collect::<std::collections::HashSet<&str>>()
                    .contains(group.get(0..count).unwrap())
                {
                    return Ok(Self::Sequence);
                } else {
                    return Err(HandErr::InvalidGroup);
                }
            }
            4 => Ok(Self::Kan),
            1 => Ok(Self::None),
            _ => Err(HandErr::InvalidGroup),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Suit {
    Manzu,
    Pinzu,
    Souzu,
    Wind,
    Dragon,
}

impl Suit {
    /// Parse the suit from the string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mahc::Suit;
    ///
    /// let tile_string = "9m";
    /// let actual_suit = Suit::suit_from_string(tile_string.chars().nth(1).unwrap().to_string());
    /// let expected = Ok(Suit::Manzu);
    ///
    /// assert_eq!(actual_suit, expected);
    /// ```
    pub fn suit_from_string(suit: String) -> Result<Self, HandErr> {
        match suit.as_str() {
            "s" => Ok(Self::Souzu),
            "p" => Ok(Self::Pinzu),
            "m" => Ok(Self::Manzu),
            "w" => Ok(Self::Wind),
            "d" => Ok(Self::Dragon),
            _ => Err(HandErr::InvalidSuit),
        }
    }
}

impl LimitHands {
    //TODO: MOVE THIS INTO A SUITABLE STRUCT LATER
    /// Check if the score of the hand is limited (no aotenjou).
    fn is_limit_hand(han: u16, fu: u16) -> bool {
        if han >= 5 {
            return true;
        }

        if han == 4 && fu >= 40 {
            return true;
        }

        if han == 3 && fu >= 70 {
            return true;
        }

        false
    }

    /// Calculate the limit hand type from the han and fu scores.
    pub fn get_limit_hand(han: u16, fu: u16) -> Option<Self> {
        if !Self::is_limit_hand(han, fu) {
            return None;
        }

        // TODO: Allow (3 han, 70+ fu) and (4 han, 40+ fu) to be considered manga.
        if han <= 5 {
            Some(Self::Mangan)
        } else if han <= 7 {
            return Some(Self::Haneman);
        } else if han <= 10 {
            return Some(Self::Baiman);
        } else if han <= 12 {
            return Some(Self::Sanbaiman);
        } else {
            return Some(Self::KazoeYakuman);
        }
    }

    /// Get the payment amounts.
    ///
    /// Format is as follows:
    ///
    /// - dealer_ron
    /// - dealer_tsumo
    /// - non_dealer_ron
    /// - non_dealer_tsumo_to_non_dealer
    /// - non_dealer_tsumo_to_dealer
    pub fn get_score(&self) -> Vec<u16> {
        match self {
            Self::Mangan => {
                vec![12000, 4000, 8000, 2000, 4000]
            }
            Self::Haneman => {
                let vec = Self::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    let j = i / 2;
                    out.push(i + j)
                }
                out
            }
            Self::Baiman => {
                let vec = Self::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 2)
                }
                out
            }
            Self::Sanbaiman => {
                let vec = Self::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 3)
                }
                out
            }
            Self::KazoeYakuman => {
                let vec = Self::Mangan.get_score();
                let mut out: Vec<u16> = Vec::new();
                for i in vec {
                    out.push(i * 4)
                }
                out
            }
        }
    }
}
