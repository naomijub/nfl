import 'package:flutter/material.dart';
import 'package:frontend/Views/input_app.dart';

class HomePage extends StatelessWidget {
  final String title;

  const HomePage({Key key, this.title}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      floatingActionButton: null,
      appBar: AppBar(
        iconTheme: IconThemeData(color: Colors.orange[100]),
        centerTitle: true,
        title: Text(
          title,
          style: TextStyle(
            color: Colors.white,
            fontSize: 28.0,
          ),
        ),
      ),
      body: Center(
        child: Container(
          width: 600,
          height: 800,
          child: Padding(
            padding: const EdgeInsets.all(10),
            child: InputApp(),
          ),
        ),
      ),
    );
  }
}
