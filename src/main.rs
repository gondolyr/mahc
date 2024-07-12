mod calc;
mod lib;
pub mod yaku;

use clap::Parser;

/// riichi mahjong calculator tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Hand tiles
    #[clap(long, value_delimiter = ' ', num_args = 1..)]
    tiles: Option<Vec<String>>,

    /// Winning tile
    #[arg(short, long)]
    win: Option<String>,

    /// Han from dora
    #[arg(short, long)]
    dora: Option<u16>,

    /// seat wind
    #[arg(short, long, default_value = "e")]
    seat: String,

    /// prevelant wind
    #[arg(short, long, default_value = "e")]
    prev: String,

    /// is tsumo
    #[arg(short, long, default_value_t = false)]
    tsumo: bool,

    /// is riichi
    #[arg(short, long, default_value_t = false)]
    riichi: bool,

    /// honba count
    #[arg(short, long, default_value_t = 0)]
    ba: u16,

    /// calculator mode
    #[arg(short, long, default_value = None, value_delimiter = ' ', num_args = 2)]
    manual: Option<Vec<u16>>,
}

pub fn parse_calculator(args: &Args) -> Result<String, calc::CalculatorErrors> {
    let honba = args.ba;
    let hanandfu = args.manual.clone().unwrap();
    let scores = calc::calculate(&hanandfu, honba);
    match scores {
        Ok(o) => Ok(format!(
            "Dealer: {} ({})\nnon-dealer: {} ({}/{})",
            o[0], o[1], o[2], o[3], o[4]
        )),
        Err(e) => Err(e),
    }
}
pub fn parse_hand(args: &Args) -> Result<String, calc::CalculatorErrors> {
    calc::get_hand_score(
        args.tiles.clone().unwrap(),
        args.win.clone().unwrap(),
        args.dora.clone().unwrap(),
        args.seat.clone(),
        args.prev.clone(),
        args.tsumo,
        args.riichi,
    );
    todo!()
}

fn main() {
    let args = Args::parse();
    if args.manual != None {
        let result = parse_calculator(&args);
        match result {
            Ok(o) => {
                println!("{}", o);
            }
            Err(e) => {
                println!("Error: {:?}", e.to_string());
            }
        }
    } else {
        let result = parse_hand(&args);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn yaku_yakuhai() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 1);
        let out = lib::Hand::new(
            vec![
                "EEEEwo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "rrrd".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 2);
        let out = lib::Hand::new(
            vec![
                "3333mo".to_string(),
                "WWWm".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 1);
        let out = lib::Hand::new(
            vec![
                "3333mo".to_string(),
                "222m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_yakuhai(), 0);
    }

    #[test]
    fn yaku_ryanpeikou() {
        let out = lib::Hand::new(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "789m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "es".to_string(),
            "ww".to_string(),
        )
        .unwrap();
        //is open
        assert_eq!(out.is_ryanpeikou(), true);

        let out = lib::Hand::new(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "678m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "es".to_string(),
            "ww".to_string(),
        )
        .unwrap();
        //is open
        assert_eq!(out.is_ryanpeikou(), false);
    }

    #[test]
    fn yaku_iipeko() {
        let out = lib::Hand::new(
            vec![
                "123s".to_string(),
                "123s".to_string(),
                "789m".to_string(),
                "789m".to_string(),
                "77m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert_eq!(out.is_iipeikou(), false);

        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        //is open
        assert_eq!(out.is_iipeikou(), false);

        let out = lib::Hand::new(
            vec![
                "rrrd".to_string(),
                "234m".to_string(),
                "22s".to_string(),
                "234m".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_iipeikou(), true);
    }

    #[test]
    fn yaku_tanyao() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "5555mo".to_string(),
                "22s".to_string(),
                "8888s".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_tanyao(), false);
        let out = lib::Hand::new(
            vec![
                "333mo".to_string(),
                "5555mo".to_string(),
                "11s".to_string(),
                "8888s".to_string(),
                "678m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_tanyao(), false);
        let out = lib::Hand::new(
            vec![
                "555mo".to_string(),
                "678p".to_string(),
                "22s".to_string(),
                "333s".to_string(),
                "345m".to_string(),
            ],
            "4m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.is_tanyao(), true);
    }

    #[test]
    fn fu_calc_simpleopenkan_simpleclosedkan() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "5555mo".to_string(),
                "11s".to_string(),
                "8888s".to_string(),
                "789m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 60);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::NonSimpleOpenTriplet,
                lib::Fu::SimpleOpenKan,
                lib::Fu::SimpleClosedKan,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_calc_edge_wait() {
        let out = lib::Hand::new(
            vec![
                "555po".to_string(),
                "234m".to_string(),
                "11s".to_string(),
                "rrrdo".to_string(),
                "789m".to_string(),
            ],
            "7m".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 30);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::SimpleOpenTriplet,
                lib::Fu::NonSimpleOpenTriplet,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn random_fu() {
        let out = lib::Hand::new(
            vec![
                "rrrdo".to_string(),
                "567m".to_string(),
                "567p".to_string(),
                "55s".to_string(),
                "456s".to_string(),
            ],
            "6s".to_string(),
            "Es".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 30);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::NonSimpleOpenTriplet,
            ]
        );
    }

    #[test]
    fn fu_cal_middle_wait() {
        let out = lib::Hand::new(
            vec![
                "123mo".to_string(),
                "rrrrdo".to_string(),
                "EEEEw".to_string(),
                "WWw".to_string(),
                "456p".to_string(),
            ],
            "5p".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 80);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::NonSimpleOpenKan,
                lib::Fu::NonSimpleClosedKan,
                lib::Fu::Toitsu,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_kans_seat_wind() {
        let out = lib::Hand::new(
            vec![
                "123mo".to_string(),
                "rrrrdo".to_string(),
                "456po".to_string(),
                "EEEEw".to_string(),
                "WWw".to_string(),
            ],
            "Ww".to_string(),
            "Ew".to_string(),
            "Ww".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 80);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::NonSimpleOpenKan,
                lib::Fu::NonSimpleClosedKan,
                lib::Fu::Toitsu,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn fu_cal_nontimple_closed_trip() {
        let out = lib::Hand::new(
            vec![
                "111mo".to_string(),
                "rrrd".to_string(),
                "345s".to_string(),
                "11s".to_string(),
                "EEEw".to_string(),
            ],
            "Ew".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(false).0, 40);
        assert_eq!(
            out.calculate_fu(false).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::NonSimpleOpenTriplet,
                lib::Fu::NonSimpleClosedTriplet,
                lib::Fu::NonSimpleOpenTriplet
            ]
        );
    }

    #[test]
    fn fu_cal_tsu_singlewait_simple_trip_closed_simple_trip_closed_nonsimple_kan() {
        let out = lib::Hand::new(
            vec![
                "444m".to_string(),
                "789p".to_string(),
                "555so".to_string(),
                "rrrrd".to_string(),
                "11s".to_string(),
            ],
            "1s".to_string(),
            "Ew".to_string(),
            "Ew".to_string(),
        )
        .unwrap();
        assert_eq!(out.calculate_fu(true).0, 70);
        assert_eq!(
            out.calculate_fu(true).1,
            [
                lib::Fu::BasePoints,
                lib::Fu::Tsumo,
                lib::Fu::SimpleClosedTriplet,
                lib::Fu::SimpleOpenTriplet,
                lib::Fu::NonSimpleClosedKan,
                lib::Fu::SingleWait,
            ]
        );
    }

    #[test]
    fn invalid_group_sequence_not_in_order() {
        let out = lib::Hand::new(
            vec![
                "135m".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "Sw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_group_size_too_small() {
        let out = lib::Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "Sw".to_string(),
                "ShSo".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_group_size_too_big() {
        let out = lib::Hand::new(
            vec![
                "SSSSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "ShSo".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidGroup);
    }

    #[test]
    fn invalid_suit() {
        let out = lib::Hand::new(
            vec![
                "hhho".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidSuit);
    }

    #[test]
    fn identify_pair() {
        let out = lib::Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.pairs()[0].value, "S");
        assert_eq!(out.pairs()[0].group_type, lib::GroupType::Pair);
        assert_eq!(out.pairs()[0].suit, lib::Suit::Wind);
        assert_eq!(out.pairs()[0].isopen, false);
    }

    #[test]
    fn hand_too_small() {
        let out = lib::Hand::new(
            vec!["SSSw".to_string()],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidShape);
        let out = lib::Hand::new(
            vec!["SSSw".to_string()],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        println!("{:?}", out);
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidShape);
    }
    #[test]
    fn hand_too_big() {
        let out = lib::Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        );
        assert_eq!(out.unwrap_err(), lib::HandErr::InvalidShape);
    }

    #[test]
    fn identify_tilegroup_closed_wind_trip_SSS() {
        let out = lib::Hand::new(
            vec![
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.triplets()[0].value, "S");
        assert_eq!(out.triplets()[0].group_type, lib::GroupType::Triplet);
        assert_eq!(out.triplets()[0].suit, lib::Suit::Wind);
        assert_eq!(out.triplets()[0].isopen, false);
    }

    #[test]
    fn identify_tilegroup_open_wind_kan_EEEE() {
        let out = lib::Hand::new(
            vec![
                "EEEEwo".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.kans()[0].value, "E");
        assert_eq!(out.kans()[0].group_type, lib::GroupType::Kan);
        assert_eq!(out.kans()[0].suit, lib::Suit::Wind);
        assert_eq!(out.kans()[0].isopen, true);
    }

    #[test]
    fn identify_tilegroup_closed_dragon_kan_RRRR() {
        let out = lib::Hand::new(
            vec![
                "rrrrd".to_string(),
                "rrrrd".to_string(),
                "SSw".to_string(),
                "rrrrd".to_string(),
                "rrrrd".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.kans()[0].value, "r");
        assert_eq!(out.kans()[0].group_type, lib::GroupType::Kan);
        assert_eq!(out.kans()[0].suit, lib::Suit::Dragon);
        assert_eq!(out.kans()[0].isopen, false);
    }

    #[test]
    fn identify_tilegroup_closed_manzu_trip_111() {
        let out = lib::Hand::new(
            vec![
                "111m".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.triplets()[0].value, "1");
        assert_eq!(out.triplets()[0].group_type, lib::GroupType::Triplet);
        assert_eq!(out.triplets()[0].suit, lib::Suit::Manzu);
        assert_eq!(out.triplets()[0].isopen, false);
    }

    #[test]
    fn identify_tilegroup_closed_souzu_seq_789() {
        let out = lib::Hand::new(
            vec![
                "789s".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.sequences()[0].value, "7");
        assert_eq!(out.sequences()[0].group_type, lib::GroupType::Sequence);
        assert_eq!(out.sequences()[0].suit, lib::Suit::Souzu);
        assert_eq!(out.sequences()[0].isopen, false);
    }

    #[test]
    fn identify_tilegroup_open_pinzu_234() {
        let out = lib::Hand::new(
            vec![
                "234po".to_string(),
                "SSSw".to_string(),
                "SSw".to_string(),
                "SSSw".to_string(),
                "SSSw".to_string(),
            ],
            "3s".to_string(),
            "3s".to_string(),
            "3s".to_string(),
        )
        .unwrap();
        assert_eq!(out.sequences()[0].value, "2");
        assert_eq!(out.sequences()[0].group_type, lib::GroupType::Sequence);
        assert_eq!(out.sequences()[0].suit, lib::Suit::Pinzu);
        assert_eq!(out.sequences()[0].isopen, true);
    }

    #[test]
    fn no_han_for_calc() {
        let args = Args::parse_from(&["", "--manual", "0", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), calc::CalculatorErrors::NoHan);
    }

    #[test]
    fn no_fu_for_calc() {
        let args = Args::parse_from(&["", "--manual", "4", "0", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(out.unwrap_err(), calc::CalculatorErrors::NoFu);
    }

    #[test]
    fn valid_calc_input() {
        let args = Args::parse_from(&["", "--manual", "4", "30", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 12500 (4200)\nnon-dealer: 8600 (2300/4200)".to_string())
        );
    }
    #[test]
    fn han_1_fu_30_calc() {
        let args = Args::parse_from(&["", "--manual", "1", "30"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 1500 (500)\nnon-dealer: 1000 (300/500)".to_string())
        );
    }
    #[test]
    fn han_2_fu_80_calc() {
        let args = Args::parse_from(&["", "--manual", "2", "80"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 7700 (2600)\nnon-dealer: 5200 (1300/2600)".to_string())
        );
    }
    #[test]
    fn han_3_mangan_calc() {
        let args = Args::parse_from(&["", "--manual", "3", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)".to_string())
        );
    }
    #[test]
    fn han_4_mangan_calc() {
        let args = Args::parse_from(&["", "--manual", "4", "60", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)".to_string())
        );
    }
    #[test]
    fn han_5_mangan_calc() {
        let args = Args::parse_from(&["", "--manual", "5", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 12900 (4300)\nnon-dealer: 8900 (2300/4300)".to_string())
        );
    }
    #[test]
    fn haneman_calc() {
        let args = Args::parse_from(&["", "--manual", "6", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)".to_string())
        );
        let args = Args::parse_from(&["", "--manual", "7", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 18900 (6300)\nnon-dealer: 12900 (3300/6300)".to_string())
        );
    }
    #[test]
    fn baiman_calc() {
        let args = Args::parse_from(&["", "--manual", "8", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)".to_string())
        );
        let args = Args::parse_from(&["", "--manual", "9", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)".to_string())
        );
        let args = Args::parse_from(&["", "--manual", "10", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 24900 (8300)\nnon-dealer: 16900 (4300/8300)".to_string())
        );
    }
    #[test]
    fn sanbaiman_calc() {
        let args = Args::parse_from(&["", "--manual", "11", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)".to_string())
        );
        let args = Args::parse_from(&["", "--manual", "12", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 36900 (12300)\nnon-dealer: 24900 (6300/12300)".to_string())
        );
    }
    #[test]
    fn kazoeyakuman_calc() {
        let args = Args::parse_from(&["", "--manual", "13", "70", "--ba", "3"]);
        let out = parse_calculator(&args);
        assert_eq!(
            out.unwrap(),
            ("Dealer: 48900 (16300)\nnon-dealer: 32900 (8300/16300)".to_string())
        );
    }
}
