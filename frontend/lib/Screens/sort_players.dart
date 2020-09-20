import 'package:flutter/material.dart';
import 'package:frontend/Screens/list_all_palyers.dart';
import 'package:frontend/Screens/sorted_players.dart';
import 'package:frontend/Views/nfl_scaffold.dart';
import 'package:frontend/bloc/info_bloc.dart';

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
          width: 480,
          child: Padding(
            padding: const EdgeInsets.all(8.0),
            child: ListView(
              padding: EdgeInsets.all(4),
              children: [
                Text(
                  'Sort Players By',
                  style: TextStyle(color: Colors.blue, fontSize: 24),
                ),
                Divider(
                  height: 1,
                  thickness: 2,
                ),
                Row(
                  mainAxisAlignment: MainAxisAlignment.start,
                  children: [
                    Radio(
                      key: Key('None'),
                      value: SortBy.None,
                      activeColor: Colors.red[900],
                      groupValue: _sort,
                      onChanged: (newValue) {
                        setState(() {
                          _sort = newValue;
                        });
                      },
                    ),
                    Expanded(
                      child: Text(
                        'No Sorting',
                        style: TextStyle(color: Colors.indigo, fontSize: 18),
                      ),
                    ),
                  ],
                ),
                Row(
                  children: [
                    Radio(
                      key: Key('LR'),
                      value: SortBy.LONGEST_RUSH,
                      activeColor: Colors.red[900],
                      groupValue: _sort,
                      onChanged: (newValue) {
                        setState(() {
                          _sort = newValue;
                        });
                      },
                    ),
                    Expanded(
                      child: Text(
                        'Longest Rush',
                        style: TextStyle(color: Colors.indigo, fontSize: 18),
                      ),
                    ),
                  ],
                ),
                Row(
                  children: [
                    Radio(
                      key: Key('TRT'),
                      value: SortBy.TOTAL_RUSHING_TOUCHDOWNS,
                      activeColor: Colors.red[900],
                      groupValue: _sort,
                      onChanged: (newValue) {
                        setState(() {
                          _sort = newValue;
                        });
                      },
                    ),
                    Expanded(
                      child: Text(
                        'Total Rushing Touchdowns',
                        style: TextStyle(color: Colors.indigo, fontSize: 18),
                      ),
                    ),
                  ],
                ),
                Row(
                  children: [
                    Radio(
                      key: Key('TRY'),
                      value: SortBy.TOTAL_RUSHING_YARDS,
                      activeColor: Colors.red[900],
                      groupValue: _sort,
                      onChanged: (newValue) {
                        setState(() {
                          _sort = newValue;
                        });
                      },
                    ),
                    Expanded(
                      child: Text(
                        'Total Rushing Yards',
                        style: TextStyle(color: Colors.indigo, fontSize: 18),
                      ),
                    ),
                  ],
                ),
                Divider(
                  height: 8,
                ),
                Text(
                  'Order Players',
                  style: TextStyle(color: Colors.blue, fontSize: 24),
                ),
                Divider(
                  height: 1,
                  thickness: 2,
                ),
                Row(
                  children: [
                    Radio(
                      key: Key('asc'),
                      value: Order.ASC,
                      activeColor: Colors.red[900],
                      groupValue: _order,
                      onChanged: (newValue) {
                        setState(() {
                          _order = newValue;
                        });
                      },
                    ),
                    Expanded(
                      child: Text(
                        'Asc Order',
                        style: TextStyle(color: Colors.indigo, fontSize: 18),
                      ),
                    ),
                  ],
                ),
                Row(
                  children: [
                    Radio(
                      key: Key('desc'),
                      value: Order.DESC,
                      activeColor: Colors.red[900],
                      groupValue: _order,
                      onChanged: (newValue) {
                        setState(() {
                          _order = newValue;
                        });
                      },
                    ),
                    Expanded(
                      child: Text(
                        'Desc Order',
                        style: TextStyle(color: Colors.indigo, fontSize: 18),
                      ),
                    ),
                  ],
                ),
                Padding(
                  padding: const EdgeInsets.only(top: 16.0),
                  child: SizedBox(
                    height: 50,
                    width: double.maxFinite,
                    child: RaisedButton(
                      key: Key('proceed'),
                      disabledColor: Colors.grey,
                      textColor: Colors.white,
                      color: Colors.blue,
                      child: Text(
                        'Search',
                        style: TextStyle(fontSize: 20),
                      ),
                      onPressed: () {
                        _sort == SortBy.None
                            ? Navigator.of(context).push(
                                MaterialPageRoute(
                                  builder: (context) => ListAllPlayers(),
                                ),
                              )
                            : Navigator.of(context).push(
                                MaterialPageRoute(
                                  builder: (context) => SortedPlayers(),
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
      ),
    );
  }
}
