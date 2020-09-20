import 'package:flutter_test/flutter_test.dart';
import 'package:frontend/bloc/info_bloc.dart';

void main() {
  test('query params as str', () {
    new InfoBloc();
    InfoBloc.queryFields = <String>['hello', 'world'];

    expect(InfoBloc.getQueryFieldsStr(), 'hello, world');
  });

  test('Sorting', () {
    new InfoBloc();
    InfoBloc.sorting = SortBy.LONGEST_RUSH;

    expect(InfoBloc.getSorting(), 'LONGEST_RUSH');
  });

  test('Order', () {
    new InfoBloc();
    InfoBloc.order = Order.ASC;

    expect(InfoBloc.getOrder(), 'ASC');
  });
}
