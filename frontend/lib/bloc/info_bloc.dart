import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class InfoBloc extends ChangeNotifier {
  static final String uri = 'http://localhost:4000/graphql';
  static int page = 0;
  static int perPage = 10;
  static String name = '';
  static List<String> queryFields = new List();

  static InfoBloc of(BuildContext context, {bool listen = true}) =>
      Provider.of<InfoBloc>(context, listen: listen);

  static pageInfo(int _perPage, int _page) {
    page = _page;
    perPage = _perPage;
  }

  static pageInfoWithName(int _perPage, int _page, String _name) {
    page = _page;
    perPage = _perPage;
    name = _name;
  }

  static String getQueryFieldsStr() {
    return queryFields.join(', ');
  }
}
