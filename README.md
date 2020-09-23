# NFL Rushing Challange

## Challenge Background
We have sets of records representing football players' rushing statistics. All records have the following attributes:
* `Player` (Player's name)
* `Team` (Player's team abbreviation)
* `Pos` (Player's postion)
* `Att/G` (Rushing Attempts Per Game Average)
* `Att` (Rushing Attempts)
* `Yds` (Total Rushing Yards)
* `Avg` (Rushing Average Yards Per Attempt)
* `Yds/G` (Rushing Yards Per Game)
* `TD` (Total Rushing Touchdowns)
* `Lng` (Longest Rush -- a `T` represents a touchdown occurred)
* `1st` (Rushing First Downs)
* `1st%` (Rushing First Down Percentage)
* `20+` (Rushing 20+ Yards Each)
* `40+` (Rushing 40+ Yards Each)
* `FUM` (Rushing Fumbles)

In this repo is a sample data file [`rushing.json`](/rushing.json).

#### Challenge Requirements
1. Create a web app. This must be able to do the following steps
    1. Create a webpage which displays a table with the contents of [`rushing.json`](/rushing.json)
    2. The user should be able to sort the players by _Total Rushing Yards_, _Longest Rush_ and _Total Rushing Touchdowns_
    3. The user should be able to filter by the player's name
    4. The user should be able to download the sorted data as a CSV, as well as a filtered subset
    
2. The system should be able to potentially support larger sets of data on the order of 10k records.

3. Update the section `Installation and running this solution` in the README file explaining how to run your code


## Installation and running this solution

To start the project is necessary to have `make` available and `docker-compose`, this way the project can be executed with `make run`, else `docker-compose up` can do the trick.
1. Open your browser at `http://localhost:5001`.
2. First screen allows you to select `perPage` and `page` required attributes, as well as optional `name` to search.
3. Second screen allows you to select all fields you wish to see in the report.
4. Third screen, if `name` was not inserted, will allow you to select a sorting method.
5. Forth screen is the Table report.
6. To download CSV you should click in the blue button at the bottom right.

## Considerations
* For sorting `Longest Rush`, I considered a touchdown a bonus of 100 points.
* Did not use flutter graphql client due to configuration time. Same for flutter web specific tests.
* I did not focus on pretty frontends, just functionality.
* Backed was developed in Rust with Actix for web server and Juniper for GraphQL support. The project is divided in `main`, where the server is started and files loaded, `reader` IO related module, `model` is the module responsible for `Player` and `Errors`, and `web` is the module containing GraphQL queries and routes configuration.
* Frontend is a little more complex. It starts with `main` that runs the `App`, that it call `Screens` where all screens are located, `Views` is where shared `Screen` parts, like scaffold, are located, common components are stored in `components`, `bloc` is a where the app data is being stored, and `repository` is where `http` and `csv` functions are stored.
* Due to disponibility reasons, query `sortByName` doesn't have its own frontend.

### Tests
To run tests just execute `cargo test`.

### Benchmarks
Benchmarks are done via `criterion = "0.3"` and can be obtained by `cargo bench`.

**read_json** microbenchmark:
```
rushing.json   time:   [2.2553 ms 2.2586 ms 2.2622 ms]
```

