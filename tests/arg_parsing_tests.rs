use fortune::parse_name_from_args;

#[test]
fn parse_name_defaults_to_you_when_no_args_are_present() {
    let name = parse_name_from_args(std::iter::empty::<&str>());

    assert_eq!(name, "you");
}

#[test]
fn parse_name_reads_value_after_name_flag() {
    let name = parse_name_from_args(["--name", "Avery"]);

    assert_eq!(name, "Avery");
}

#[test]
fn parse_name_defaults_to_you_when_name_value_is_missing() {
    let name = parse_name_from_args(["--name"]);

    assert_eq!(name, "you");
}

#[test]
fn parse_name_keeps_empty_string_value() {
    let name = parse_name_from_args(["--name", ""]);

    assert_eq!(name, "");
}

#[test]
fn parse_name_uses_first_name_flag_when_repeated() {
    let name = parse_name_from_args(["--name", "first", "--name", "second"]);

    assert_eq!(name, "first");
}

#[test]
fn parse_name_ignores_unknown_flags() {
    let name = parse_name_from_args(["--verbose", "--count", "4"]);

    assert_eq!(name, "you");
}
