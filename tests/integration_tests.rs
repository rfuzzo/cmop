#[cfg(test)]
mod integration_tests {
    use rand::{seq::SliceRandom, thread_rng};

    use plox::{parser::*, sorter::*, *};

    fn init() {
        let env = env_logger::Env::default()
            .default_filter_or(log_level_to_str(ELogLevel::Info))
            .default_write_style_or("always");
        let _ = env_logger::Builder::from_env(env).is_test(true).try_init();
    }

    #[test]
    fn test_read_mods() {
        init();

        let mods_path = "./tests/modlist.txt";
        assert_eq!(
            read_file_as_list(mods_path),
            vec![
                "a.archive",
                "b.archive",
                "c.archive",
                "d.archive",
                "e.archive"
            ]
        )
    }

    #[test]
    fn test_verify_order_rules() {
        init();

        let rules = new_cyberpunk_parser()
            .parse_rules_from_path("./tests/plox/rules_order.txt")
            .expect("rule parse failed");
        let order = get_order_rules(&rules);

        let mods = debug_get_mods_from_rules(&order);

        assert!(
            new_unstable_sorter().topo_sort(&mods, &order).is_ok(),
            "rules contain a cycle"
        )
    }

    #[test]
    fn test_parse_notes() {
        init();

        let rules = new_cyberpunk_parser()
            .parse_rules_from_path("./tests/plox/rules_note.txt")
            .expect("rule parse failed");

        assert_eq!(15, rules.len());
    }

    #[test]
    fn test_verify_mlox_base_rules() {
        init();

        let parser = new_tes3_parser();
        let rules = parser
            .parse_rules_from_path("./tests/mlox/mlox_base.txt")
            .expect("rule parse failed");
        let order = get_order_rules(&rules);

        let mut rng = thread_rng();
        let mut mods = debug_get_mods_from_rules(&order);

        for m in &mods {
            assert!(parser.ends_with_vec3(m));
        }

        mods.shuffle(&mut rng);

        assert!(
            new_unstable_sorter().topo_sort(&mods, &order).is_ok(),
            "rules contain a cycle"
        )
    }

    #[test]
    fn test_verify_mlox_user_rules() {
        init();

        let parser = new_tes3_parser();
        let rules = parser
            .parse_rules_from_path("./tests/mlox/mlox_user.txt")
            .expect("rule parse failed");
        let order = get_order_rules(&rules);

        //let mut rng = thread_rng();
        let mods = debug_get_mods_from_rules(&order);

        for m in &mods {
            assert!(parser.ends_with_vec3(m));
        }

        //mods.shuffle(&mut rng);

        assert!(
            new_unstable_sorter().topo_sort(&mods, &order).is_ok(),
            "rules contain a cycle"
        )
    }

    #[test]
    fn test_gather_mods() {
        init();

        let root_path = "./tests";

        let mods = gather_mods(&root_path, ESupportedGame::Cyberpunk);
        assert_eq!(
            mods,
            vec![
                "a.archive".to_owned(),
                "b.archive".into(),
                "c.archive".into()
            ]
        )
    }
}
