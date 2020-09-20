import 'package:flutter/material.dart';

import 'Screens/home_page.dart';
import 'bloc/info_bloc.dart';

void main() {
  runApp(NflApp());
}

class NflApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final InfoBloc _ = InfoBloc();
    return MaterialApp(
      title: 'NFL',
      theme: ThemeData(
        visualDensity: VisualDensity.adaptivePlatformDensity,
      ),
      home: HomePage(title: 'NFL Rushing APP'),
    );
  }
}
