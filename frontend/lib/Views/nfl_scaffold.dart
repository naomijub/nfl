import 'package:flutter/material.dart';

class NflScaffold extends StatelessWidget {
  @required
  final String title;
  @required
  final Widget body;
  final FloatingActionButton button;

  const NflScaffold({Key key, this.title, this.body, this.button})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      floatingActionButton: button != null ? button : null,
      appBar: AppBar(
        iconTheme: IconThemeData(color: Colors.orange[100]),
        centerTitle: true,
        backgroundColor: Colors.orange[700],
        title: Text(
          title,
          style: TextStyle(
            color: Colors.white,
            fontSize: 28.0,
          ),
        ),
      ),
      body: body,
    );
  }
}
