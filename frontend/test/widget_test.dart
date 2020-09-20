// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility that Flutter provides. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'package:flutter_test/flutter_test.dart';

import 'package:frontend/main.dart';

void main() {
  testWidgets('Counter increments smoke test', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(NflApp());

    // Verify that our counter starts at 0.
    expect(find.text('Players per Page'), findsOneWidget);
    expect(find.text('Pagination'), findsOneWidget);
    expect(find.text('Player Name'), findsOneWidget);
    expect(find.text('Not Valid'), findsNothing);
  });
}
