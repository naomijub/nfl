import 'package:flutter/material.dart';
import 'package:frontend/Screens/players_by_name.dart';
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
                height: 800,
                child: ListView.builder(
                  itemCount: 15,
                  itemBuilder: (BuildContext context, int index) {
                    return Container(
                      child: LabelCheckbox(
                        padding: const EdgeInsets.all(4),
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
                padding: const EdgeInsets.only(top: 16.0),
                child: SizedBox(
                  width: double.maxFinite,
                  child: RaisedButton(
                    key: Key('proceed'),
                    disabledColor: Colors.grey,
                    textColor: Colors.white,
                    color: Colors.blue,
                    child: InfoBloc.name.isNotEmpty
                        ? Text('Search')
                        : Text('Proceed'),
                    onPressed: _areSelected.any((e) => e == true)
                        ? () {
                            final searchValues = zip([options, _areSelected])
                                .where((e) => e[1])
                                .map((e) => e[0])
                                .toList();

                            Navigator.of(context).push(
                              MaterialPageRoute(
                                builder: (context) =>
                                    PlayersByName(searchValues: searchValues),
                              ),
                            );
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
