use nfl_rushing::{model::json::Player, reader::read_json};

#[test]
fn read_json_can_read_file_to_vec_player() {
    let actual = read_json("test.json").unwrap();
    let expected = vec![Player {
        name: String::from("Johnny Holton"),
        team: String::from("OAK"),
        position: String::from("WR"),
        avg_attempts_per_game: 0.4f64,
        attemps: 6u64,
        total_rushing_yards: 43i64,
        average_rushing_yards_per_attemp: 7.2f64,
        rushing_yards_per_game: 2.9f64,
        total_rushing_touchdowns: 0u64,
        largest_rust: String::from("29"),
        rushing_first_downs: 2f64,
        rushing_first_downs_percentage: 33.3f64,
        rushing_20_yards: 1f64,
        rushing_40_yards: 0f64,
        rushing_fumbles: 100000u64,
    }];

    assert_eq!(actual, expected);
}

#[test]
#[should_panic(expected = "1st")]
fn read_json_throws_error_at_field_1st_equals_true() {
    let _ = read_json("error.json").unwrap();
}
