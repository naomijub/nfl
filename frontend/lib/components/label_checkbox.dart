import 'package:flutter/material.dart';

class LabelCheckbox extends StatelessWidget {
  const LabelCheckbox({
    this.label,
    this.padding,
    this.value,
    this.onChanged,
  });

  final String label;
  final EdgeInsets padding;
  final bool value;
  final Function onChanged;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: padding,
      child: Row(
        children: <Widget>[
          Expanded(
            child: RichText(
              text: TextSpan(
                text: label,
                style: TextStyle(
                  color: Colors.red[900],
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                ),
                recognizer: null,
              ),
            ),
          ),
          Checkbox(
            key: Key('checkbox'),
            value: value,
            activeColor: Colors.white,
            checkColor: Colors.red[900],
            onChanged: (bool newValue) {
              onChanged(newValue);
            },
          ),
        ],
      ),
    );
  }
}
