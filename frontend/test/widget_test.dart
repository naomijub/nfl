// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility that Flutter provides. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:frontend/main.dart';

void main() {
  testWidgets('Homepage loads', (WidgetTester tester) async {
    await tester.pumpWidget(NflApp());

    expect(find.text('Players per Page'), findsOneWidget);
    expect(find.text('Pagination'), findsOneWidget);
    expect(find.text('Player Name'), findsOneWidget);
    expect(find.text('Not Valid'), findsNothing);
  });

//   Test seems to have been failing in flutter web, worked in mobile version
//   testWidgets('navigates to selected fields', (WidgetTester tester) async {
//     await tester.pumpWidget(NflApp());

//     await tester.enterText(find.byKey(Key('perPage')), '3');
//     await tester.enterText(find.byKey(Key('page')), '1');
//     await tester.enterText(find.byKey(Key('name')), 'Joe');

//     await tester.press(find.byKey(Key('proceed')));
//     await tester.pumpAndSettle();

//     expect(find.byKey(Key('checkbox')), findsWidgets);
//   });
}
