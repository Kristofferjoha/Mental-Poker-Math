use backend::calculators::calculator_helpers::card_to_poker_eval_id;
use backend::poker_core::card::{Card, Rank, Suit};

fn card(rank: Rank, suit: Suit) -> Card {
    Card { rank, suit }
}

#[test]
fn card_ids_match_poker_eval_layout() {
    assert_eq!(card_to_poker_eval_id(&card(Rank::Two, Suit::Clubs)), 0);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Two, Suit::Diamonds)), 1);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Two, Suit::Hearts)), 2);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Two, Suit::Spades)), 3);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Ace, Suit::Clubs)), 48);
    assert_eq!(card_to_poker_eval_id(&card(Rank::Ace, Suit::Spades)), 51);
}

mod cors_allow_list {
    use backend::program::allowed_origins;

    fn with_env(value: Option<&str>, f: impl FnOnce()) {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        match value {
            Some(v) => std::env::set_var("CORS_ALLOWED_ORIGINS", v),
            None => std::env::remove_var("CORS_ALLOWED_ORIGINS"),
        }
        f();
        std::env::remove_var("CORS_ALLOWED_ORIGINS");
    }

    #[test]
    fn defaults_cover_production_and_local_dev() {
        with_env(None, || {
            let origins = allowed_origins().expect("defaults must parse");
            let as_str: Vec<&str> = origins.iter().filter_map(|o| o.to_str().ok()).collect();
            assert!(as_str.contains(&"https://mentalpokermath.com"));
            assert!(as_str.contains(&"http://localhost:5173"));
            assert!(as_str.contains(&"http://127.0.0.1:5173"));
        });
    }

    #[test]
    fn https_origins_survive_validation() {
        with_env(Some("https://staging.example"), || {
            let origins = allowed_origins().expect("https origin must be accepted");
            assert_eq!(origins.len(), 1);
        });
    }

    #[test]
    fn typos_are_dropped_not_silently_accepted() {
        with_env(Some("htp://localhost:5173,https://good.example/,localhost:5173,https://ok.example"), || {
            let origins = allowed_origins().expect("one good entry remains");
            let as_str: Vec<&str> = origins.iter().filter_map(|o| o.to_str().ok()).collect();
            assert_eq!(as_str, vec!["https://ok.example"]);
        });
    }

    #[test]
    fn empty_list_is_an_error_not_a_wide_open_server() {
        with_env(Some("  ,  "), || {
            assert!(allowed_origins().is_err());
        });
    }
}
