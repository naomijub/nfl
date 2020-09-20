import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:frontend/Screens/players_by_name.dart';
import 'package:frontend/bloc/info_bloc.dart';

import 'nfl_scaffold.dart';

class InputApp extends StatefulWidget {
  InputApp({Key key}) : super(key: key);

  @override
  _InputAppState createState() => _InputAppState();
}

class _InputAppState extends State<InputApp> {
  final TextEditingController _perPageController = TextEditingController();
  final TextEditingController _pageController = TextEditingController();
  final TextEditingController _nameController = TextEditingController();
  bool isSearch = false;

  Widget build(BuildContext context) {
    return NflScaffold(
      title: 'Input search fields',
      body: Padding(
        padding: const EdgeInsets.fromLTRB(16.0, 32.0, 16.0, 16.0),
        child: ListView(
          children: [
            TextField(
              controller: _perPageController,
              keyboardType: TextInputType.number,
              inputFormatters: <TextInputFormatter>[
                FilteringTextInputFormatter.digitsOnly
              ],
              decoration: InputDecoration(
                labelText: 'Players per Page',
              ),
              style: TextStyle(
                fontSize: 24.0,
              ),
            ),
            TextField(
              controller: _pageController,
              keyboardType: TextInputType.number,
              inputFormatters: <TextInputFormatter>[
                FilteringTextInputFormatter.digitsOnly
              ],
              decoration: InputDecoration(
                labelText: 'Pagination',
              ),
              style: TextStyle(
                fontSize: 24.0,
              ),
            ),
            TextField(
              controller: _nameController,
              keyboardType: TextInputType.number,
              decoration: InputDecoration(
                labelText: 'Player Name',
              ),
              style: TextStyle(
                fontSize: 24.0,
              ),
            ),
            Padding(
              padding: const EdgeInsets.only(top: 16.0),
              child: SizedBox(
                width: double.maxFinite,
                child: RaisedButton(
                  disabledColor: Colors.grey,
                  textColor: Colors.white,
                  color: Colors.blue,
                  child: isSearch ? Text('Search') : Text('Proceed'),
                  onPressed: () {
                    isSearch = _nameController.text.isEmpty;
                    final String name = _nameController.text;
                    final int page = int.parse(_pageController.text);
                    final int perPage = int.parse(_perPageController.text);

                    if (name.isEmpty) {
                      InfoBloc.pageInfo(perPage, page);
                    } else {
                      InfoBloc.pageInfoWithName(perPage, page, name);
                      Navigator.of(context).push(
                        MaterialPageRoute(
                          builder: (context) => PlayersByName(),
                        ),
                      );
                    }
                  },
                ),
              ),
            )
          ],
        ),
      ),
    );
  }
}
