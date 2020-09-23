use crate::model::{error::Error, json::Player};
use crate::web::graphql_resolvers::{Order, Sorting};

pub fn list_all_players(players: Vec<Player>, per_page: i32, page: i32) -> Vec<Player> {
    players
        .iter()
        .skip((per_page * page) as usize)
        .take(per_page as usize)
        .map(|p| p.to_owned())
        .collect::<Vec<Player>>()
}

pub fn players_by_name(
    players: Vec<Player>,
    per_page: i32,
    page: i32,
    pattern: String,
) -> Vec<Player> {
    players
        .iter()
        .filter(|p| p.name.starts_with(&pattern))
        .skip((per_page * page) as usize)
        .take(per_page as usize)
        .map(|p| p.to_owned())
        .collect::<Vec<Player>>()
}

pub fn sort_players(
    players: Vec<Player>,
    per_page: i32,
    page: i32,
    sort_by: Sorting,
    order: Order,
) -> Result<Vec<Player>, Error> {
    let mut players = players
        .iter()
        .skip((per_page * page) as usize)
        .take(per_page as usize)
        .map(|p| p.to_owned())
        .collect::<Vec<Player>>();
    sort_players_by(&mut players, sort_by, order);
    Ok(players)
}

pub fn sort_by_name(
    players: Vec<Player>,
    per_page: i32,
    page: i32,
    pattern: String,
    sort_by: Sorting,
    order: Order,
) -> Result<Vec<Player>, Error> {
    let mut players = players
        .iter()
        .filter(|p| p.name.starts_with(&pattern))
        .skip((per_page * page) as usize)
        .take(per_page as usize)
        .map(|p| p.to_owned())
        .collect::<Vec<Player>>();
    sort_players_by(&mut players, sort_by, order);
    Ok(players)
}

pub(crate) fn sort_players_by(players: &mut Vec<Player>, sort_by: Sorting, order: Order) {
    match (order, sort_by) {
        (Order::Asc, Sorting::TotalRushingYards) => players.sort_by(|a, b| {
            a.total_rushing_yards
                .partial_cmp(&b.total_rushing_yards)
                .unwrap()
        }),
        (Order::Asc, Sorting::LongestRush) => {
            players.sort_by(|a, b| a.longest_rush().partial_cmp(&b.longest_rush()).unwrap())
        }
        (Order::Asc, Sorting::TotalRushingTouchdowns) => players.sort_by(|a, b| {
            a.total_rushing_touchdowns
                .partial_cmp(&b.total_rushing_touchdowns)
                .unwrap()
        }),
        (Order::Desc, Sorting::TotalRushingYards) => players.sort_by(|b, a| {
            a.total_rushing_yards
                .partial_cmp(&b.total_rushing_yards)
                .unwrap()
        }),
        (Order::Desc, Sorting::LongestRush) => {
            players.sort_by(|b, a| a.longest_rush().partial_cmp(&b.longest_rush()).unwrap())
        }
        (Order::Desc, Sorting::TotalRushingTouchdowns) => players.sort_by(|b, a| {
            a.total_rushing_touchdowns
                .partial_cmp(&b.total_rushing_touchdowns)
                .unwrap()
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::json::Player;
    use crate::web::graphql_resolvers::{Order, Sorting};

    #[test]
    fn sort_by_td() {
        let mut players = players_mock();
        sort_players_by(&mut players, Sorting::TotalRushingTouchdowns, Order::Asc);

        let name_order = players
            .iter()
            .map(|p| p.name.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(
            name_order,
            vec!["Hello", "Olá", "Salem", "Namaste", "Marhabaan"]
        );

        sort_players_by(&mut players, Sorting::TotalRushingTouchdowns, Order::Desc);
        let name_order = players
            .iter()
            .map(|p| p.name.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(
            name_order,
            vec!["Marhabaan", "Namaste", "Salem", "Olá", "Hello"]
        );
    }

    #[test]
    fn sort_by_lr() {
        let mut players = players_mock();
        sort_players_by(&mut players, Sorting::LongestRush, Order::Asc);

        let name_order = players
            .iter()
            .map(|p| p.name.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(
            name_order,
            vec!["Hello", "Olá", "Salem", "Namaste", "Marhabaan"]
        );

        sort_players_by(&mut players, Sorting::LongestRush, Order::Desc);
        let name_order = players
            .iter()
            .map(|p| p.name.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(
            name_order,
            vec!["Marhabaan", "Namaste", "Salem", "Olá", "Hello"]
        );
    }

    #[test]
    fn sort_by_ty() {
        let mut players = players_mock();
        sort_players_by(&mut players, Sorting::TotalRushingYards, Order::Asc);

        let name_order = players
            .iter()
            .map(|p| p.name.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(
            name_order,
            vec!["Hello", "Olá", "Salem", "Namaste", "Marhabaan"]
        );

        sort_players_by(&mut players, Sorting::TotalRushingYards, Order::Desc);
        let name_order = players
            .iter()
            .map(|p| p.name.to_owned())
            .collect::<Vec<String>>();
        assert_eq!(
            name_order,
            vec!["Marhabaan", "Namaste", "Salem", "Olá", "Hello"]
        );
    }

    #[test]
    fn test_list_all_players() {
        let players = players_mock();
        let players_0 = list_all_players(players_mock(), 3, 0);
        let players_1 = list_all_players(players_mock(), 3, 1);
        let (exp_0, exp_1) = players.split_at(3);

        assert_eq!(exp_0, &players_0[..]);
        assert_eq!(exp_1, &players_1[..]);
    }

    #[test]
    fn test_players_by_name() {
        let exp: Vec<Player> = Vec::new();
        let players_0 = players_by_name(players_mock(), 3, 0, "M".to_string());
        let players_1 = players_by_name(players_mock(), 3, 1, "M".to_string());

        assert_eq!(
            vec!["Marhabaan".to_string()],
            players_0
                .iter()
                .map(|p| p.name.to_owned())
                .collect::<Vec<String>>()
        );
        assert_eq!(exp, players_1);
    }

    #[test]
    fn test_sort_players() {
        let players = sort_players(players_mock(), 3, 0, Sorting::LongestRush, Order::Asc);

        assert_eq!(
            vec!["Hello".to_string(), "Olá".to_string(), "Salem".to_string()],
            players
                .unwrap()
                .iter()
                .map(|p| p.name.to_owned())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_sort_players_by() {
        let players = sort_by_name(
            players_mock(),
            3,
            0,
            "Sa".to_string(),
            Sorting::LongestRush,
            Order::Asc,
        );

        assert_eq!(
            vec!["Salem".to_string()],
            players
                .unwrap()
                .iter()
                .map(|p| p.name.to_owned())
                .collect::<Vec<String>>()
        );
    }

    fn players_mock() -> Vec<Player> {
        vec![
            Player {
                name: String::from("Olá"),
                team: String::from("Mundo"),
                position: String::from("FW"),
                avg_attempts_per_game: 2f64,
                attemps: 2i32,
                total_rushing_yards: 2i32,
                average_rushing_yards_per_attemp: 2f64,
                rushing_yards_per_game: 2f64,
                total_rushing_touchdowns: 2i32,
                longest_rush: String::from("2"),
                rushing_first_downs: 2f64,
                rushing_first_downs_percentage: 2f64,
                rushing_20_yards: 2f64,
                rushing_40_yards: 2f64,
                rushing_fumbles: 2i32,
            },
            Player {
                name: String::from("Hello"),
                team: String::from("World"),
                position: String::from("FW"),
                avg_attempts_per_game: 1f64,
                attemps: 1i32,
                total_rushing_yards: 1i32,
                average_rushing_yards_per_attemp: 1f64,
                rushing_yards_per_game: 1f64,
                total_rushing_touchdowns: 1i32,
                longest_rush: String::from("1"),
                rushing_first_downs: 1f64,
                rushing_first_downs_percentage: 1f64,
                rushing_20_yards: 1f64,
                rushing_40_yards: 1f64,
                rushing_fumbles: 1i32,
            },
            Player {
                name: String::from("Salem"),
                team: String::from("Alem"),
                position: String::from("FW"),
                avg_attempts_per_game: 3f64,
                attemps: 3i32,
                total_rushing_yards: 3i32,
                average_rushing_yards_per_attemp: 3f64,
                rushing_yards_per_game: 3f64,
                total_rushing_touchdowns: 3i32,
                longest_rush: String::from("10"),
                rushing_first_downs: 3f64,
                rushing_first_downs_percentage: 3f64,
                rushing_20_yards: 3f64,
                rushing_40_yards: 3f64,
                rushing_fumbles: 3i32,
            },
            Player {
                name: String::from("Marhabaan"),
                team: String::from("Bialealam"),
                position: String::from("FW"),
                avg_attempts_per_game: 5f64,
                attemps: 5i32,
                total_rushing_yards: 5i32,
                average_rushing_yards_per_attemp: 5f64,
                rushing_yards_per_game: 5f64,
                total_rushing_touchdowns: 5i32,
                longest_rush: String::from("14T"),
                rushing_first_downs: 5f64,
                rushing_first_downs_percentage: 5f64,
                rushing_20_yards: 5f64,
                rushing_40_yards: 5f64,
                rushing_fumbles: 5i32,
            },
            Player {
                name: String::from("Namaste"),
                team: String::from("Duniya"),
                position: String::from("FW"),
                avg_attempts_per_game: 4f64,
                attemps: 4i32,
                total_rushing_yards: 4i32,
                average_rushing_yards_per_attemp: 4f64,
                rushing_yards_per_game: 4f64,
                total_rushing_touchdowns: 4i32,
                longest_rush: String::from("50"),
                rushing_first_downs: 4f64,
                rushing_first_downs_percentage: 4f64,
                rushing_20_yards: 4f64,
                rushing_40_yards: 4f64,
                rushing_fumbles: 4i32,
            },
        ]
    }
}
