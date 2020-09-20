// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility that Flutter provides. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:frontend/Screens/select_fields.dart';

import 'package:frontend/main.dart';

void main() {
  testWidgets('Homepage loads', (WidgetTester tester) async {
    await tester.pumpWidget(NflApp());

    expect(find.text('Players per Page'), findsOneWidget);
    expect(find.text('Pagination'), findsOneWidget);
    expect(find.text('Player Name'), findsOneWidget);
    expect(find.text('Not Valid'), findsNothing);
  });

  testWidgets('Selected fields', (WidgetTester tester) async {
    await tester.pumpWidget(MaterialApp(home: SelectFields()));

    expect(find.byKey(Key('checkbox')), findsWidgets);
    expect(find.byKey(Key('proceed')), findsNothing);

    await tester.press(find.byKey(Key('checkbox')).first);
    await tester.pumpAndSettle();
  });
}
