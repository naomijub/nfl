import 'package:flutter/material.dart';
import 'package:frontend/Screens/players_by_name.dart';
import 'package:frontend/Views/nfl_scaffold.dart';

enum Order { ASC, DESC }
enum SortBy {
  None,
  TOTAL_RUSHING_YARDS,
  LONGEST_RUSH,
  TOTAL_RUSHING_TOUCHDOWNS
}

class SortPlayers extends StatefulWidget {
  SortPlayers({Key key}) : super(key: key);

  @override
  _SortPlayersState createState() => _SortPlayersState();
}

class _SortPlayersState extends State<SortPlayers> {
  Order _order = Order.ASC;
  SortBy _sort = SortBy.None;
  @override
  Widget build(BuildContext context) {
    return NflScaffold(
      title: 'Select Sorting Mode',
      body: Center(
        child: Container(
          width: 600,
          child: ListView(
            children: [
              Center(
                child: SizedBox(
                  width: 600,
                  child: Row(
                    children: [
                      Column(
                        children: [
                          Text(
                            'Sort Players By',
                            style: TextStyle(color: Colors.blue, fontSize: 24),
                          ),
                          Row(
                            children: [
                              Text(
                                'None',
                                style: TextStyle(
                                    color: Colors.blue[900], fontSize: 18),
                              ),
                              Radio(
                                value: SortBy.None,
                                groupValue: _sort,
                                onChanged: (newValue) {
                                  setState(() {
                                    _sort = newValue;
                                  });
                                },
                              ),
                            ],
                          ),
                          Row(
                            children: [
                              Text(
                                'Longest Rush',
                                style: TextStyle(
                                    color: Colors.blue[900], fontSize: 18),
                              ),
                              Radio(
                                value: SortBy.LONGEST_RUSH,
                                groupValue: _sort,
                                onChanged: (newValue) {
                                  setState(() {
                                    _sort = newValue;
                                  });
                                },
                              ),
                            ],
                          ),
                          Row(
                            children: [
                              Text(
                                'Total Rushing Touchdowns',
                                style: TextStyle(
                                    color: Colors.blue[900], fontSize: 18),
                              ),
                              Radio(
                                value: SortBy.TOTAL_RUSHING_TOUCHDOWNS,
                                groupValue: _sort,
                                onChanged: (newValue) {
                                  setState(() {
                                    _sort = newValue;
                                  });
                                },
                              ),
                            ],
                          ),
                          Row(
                            children: [
                              Text(
                                'Total Rushing Yards',
                                style: TextStyle(
                                    color: Colors.blue[900], fontSize: 18),
                              ),
                              Radio(
                                value: SortBy.TOTAL_RUSHING_YARDS,
                                groupValue: _sort,
                                onChanged: (newValue) {
                                  setState(() {
                                    _sort = newValue;
                                  });
                                },
                              ),
                            ],
                          )
                        ],
                      ),
                      Column(
                        children: [
                          Text(
                            'Order Players',
                            style: TextStyle(color: Colors.blue, fontSize: 24),
                          ),
                          Row(
                            children: [
                              Text(
                                'Asc',
                                style: TextStyle(
                                    color: Colors.blue[900], fontSize: 18),
                              ),
                              Radio(
                                value: Order.ASC,
                                groupValue: _order,
                                onChanged: (newValue) {
                                  setState(() {
                                    _order = newValue;
                                  });
                                },
                              ),
                            ],
                          ),
                          Row(
                            children: [
                              Text(
                                'Desc',
                                style: TextStyle(
                                    color: Colors.blue[900], fontSize: 18),
                              ),
                              Radio(
                                value: Order.DESC,
                                groupValue: _order,
                                onChanged: (newValue) {
                                  setState(() {
                                    _order = newValue;
                                  });
                                },
                              ),
                            ],
                          )
                        ],
                      )
                    ],
                  ),
                ),
              ),
              Padding(
                padding: const EdgeInsets.only(top: 16.0),
                child: SizedBox(
                  width: double.maxFinite,
                  child: RaisedButton(
                    key: Key('proceed'),
                    disabledColor: Colors.grey,
                    textColor: Colors.white,
                    color: Colors.blue,
                    child: Text(
                      'Search',
                    ),
                    onPressed: () {
                      Navigator.of(context).push(
                        MaterialPageRoute(
                          builder: (context) => PlayersByName(),
                        ),
                      );
                    },
                  ),
                ),
              )
            ],
          ),
        ),
      ),
    );
  }
}
