import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:frontend/Screens/select_fields.dart';
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

  Widget build(BuildContext context) {
    return NflScaffold(
      title: 'Input search fields',
      body: Padding(
        padding: const EdgeInsets.fromLTRB(16.0, 32.0, 16.0, 16.0),
        child: ListView(
          children: [
            TextField(
              key: Key('perPage'),
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
              key: Key('page'),
              controller: _pageController,
              keyboardType: TextInputType.number,
              inputFormatters: <TextInputFormatter>[
                FilteringTextInputFormatter.digitsOnly,
              ],
              decoration: InputDecoration(
                labelText: 'Pagination',
              ),
              style: TextStyle(
                fontSize: 24.0,
              ),
            ),
            TextField(
              key: Key('name'),
              controller: _nameController,
              keyboardType: TextInputType.number,
              decoration: InputDecoration(
                labelText: 'Player Name (Optional)',
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
                  key: Key('proceed'),
                  disabledColor: Colors.grey,
                  textColor: Colors.white,
                  color: Colors.blue,
                  child: Text('Proceed'),
                  onPressed: () {
                    final String name = _nameController.text;
                    final int page = int.parse(_pageController.text);
                    final int perPage = int.parse(_perPageController.text);

                    print(name);
                    if (name.isEmpty) {
                      InfoBloc.pageInfo(perPage, page);
                    } else {
                      InfoBloc.pageInfoWithName(perPage, page, name);
                    }

                    Navigator.of(context).push(
                      MaterialPageRoute(
                        builder: (context) => SelectFields(),
                      ),
                    );
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
