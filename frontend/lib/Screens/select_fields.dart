import 'package:flutter/material.dart';
import 'package:frontend/Screens/players_by_name.dart';
import 'package:frontend/Screens/sort_players.dart';
import 'package:frontend/Views/nfl_scaffold.dart';
import 'package:frontend/bloc/info_bloc.dart';
import 'package:frontend/components/label_checkbox.dart';
import 'package:quiver/iterables.dart';

final List<String> options = <String>[
  'name',
  'team',
  'position',
  'avgAttemptsPerGame',
  'attemps',
  'totalRushingYards',
  'averageRushingYardsPerAttemp',
  'rushingYardsPerGame',
  'totalRushingTouchdowns',
  'longestRush',
  'rushingFirstDowns',
  'rushingFirstDownsPercentage',
  'rushing20Yards',
  'rushing40Yards',
  'rushingFumbles'
];

class SelectFields extends StatefulWidget {
  SelectFields({Key key}) : super(key: key);

  @override
  _SelectFieldsState createState() => _SelectFieldsState();
}

class _SelectFieldsState extends State<SelectFields> {
  List<bool> _areSelected = <bool>[
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false
  ];

  @override
  Widget build(BuildContext context) {
    return NflScaffold(
      title: 'Select fields to query',
      body: Center(
        child: Container(
          width: 600,
          child: ListView(
            children: [
              SizedBox(
                height: 660,
                child: ListView.builder(
                  itemCount: 15,
                  itemBuilder: (BuildContext context, int index) {
                    return Container(
                      child: LabelCheckbox(
                        padding: const EdgeInsets.all(2),
                        label: options[index],
                        value: _areSelected[index],
                        onChanged: (bool newValue) {
                          setState(() {
                            _areSelected[index] = newValue;
                          });
                        },
                      ),
                    );
                  },
                ),
              ),
              Padding(
                padding: const EdgeInsets.only(top: 3.0),
                child: SizedBox(
                  height: 50,
                  width: double.maxFinite,
                  child: RaisedButton(
                    key: Key('proceed'),
                    disabledColor: Colors.grey,
                    textColor: Colors.white,
                    color: Colors.blue,
                    child: InfoBloc.name.isNotEmpty
                        ? Text('Search', style: TextStyle(fontSize: 20))
                        : Text('Proceed', style: TextStyle(fontSize: 20)),
                    onPressed: _areSelected.any((e) => e == true)
                        ? () {
                            final queryFields = zip([options, _areSelected])
                                .where((e) => e[1])
                                .map((e) => e[0])
                                .toList();
                            InfoBloc.queryFields = queryFields;

                            if (InfoBloc.name.isEmpty) {
                              Navigator.of(context).push(
                                MaterialPageRoute(
                                  builder: (context) => SortPlayers(),
                                ),
                              );
                            } else {
                              Navigator.of(context).push(
                                MaterialPageRoute(
                                  builder: (context) => PlayersByName(),
                                ),
                              );
                            }
                          }
                        : null,
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
