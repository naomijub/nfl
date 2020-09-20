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
  final _formKey = GlobalKey<FormState>();

  Widget build(BuildContext context) {
    return NflScaffold(
      title: 'Input search fields',
      body: Padding(
        padding: const EdgeInsets.fromLTRB(16.0, 32.0, 16.0, 16.0),
        child: Form(
          key: _formKey,
          child: ListView(
            children: [
              TextFormField(
                key: Key('perPage'),
                controller: _perPageController,
                inputFormatters: <TextInputFormatter>[
                  FilteringTextInputFormatter.digitsOnly
                ],
                decoration: InputDecoration(
                  labelText: 'Players per Page',
                ),
                style: TextStyle(
                  fontSize: 24.0,
                ),
                validator: (value) {
                  if (value.isEmpty) {
                    return 'Please enter a valid number';
                  }
                  return null;
                },
              ),
              TextFormField(
                key: Key('page'),
                controller: _pageController,
                inputFormatters: <TextInputFormatter>[
                  FilteringTextInputFormatter.digitsOnly
                ],
                decoration: InputDecoration(
                  labelText: 'Pagination',
                ),
                style: TextStyle(
                  fontSize: 24.0,
                ),
                validator: (value) {
                  if (value.isEmpty) {
                    return 'Please enter a valid number';
                  }
                  return null;
                },
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
                  height: 50,
                  width: double.maxFinite,
                  child: RaisedButton(
                    key: Key('proceed'),
                    disabledColor: Colors.grey,
                    textColor: Colors.white,
                    color: Colors.blue,
                    child: Text(
                      'Proceed',
                      style: TextStyle(fontSize: 20),
                    ),
                    onPressed: () {
                      final String name = _nameController.text;
                      final int page = int.parse(_pageController.text);
                      final int perPage = int.parse(_perPageController.text);

                      if (_formKey.currentState.validate()) {
                        Scaffold.of(context).showSnackBar(
                            SnackBar(content: Text('Processing Data')));
                      }

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
              ),
            ],
          ),
        ),
      ),
    );
  }
}
