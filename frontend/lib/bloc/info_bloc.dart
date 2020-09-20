import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

enum Order { ASC, DESC }
enum SortBy {
  None,
  TOTAL_RUSHING_YARDS,
  LONGEST_RUSH,
  TOTAL_RUSHING_TOUCHDOWNS
}

class InfoBloc extends ChangeNotifier {
  static final String uri = 'http://localhost:4000/graphql';
  static int page = 0;
  static int perPage = 10;
  static String name = '';
  static SortBy sorting = SortBy.None;
  static Order order = Order.ASC;
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

  static String getSorting() {
    switch (sorting) {
      case SortBy.LONGEST_RUSH:
        return 'LONGEST_RUSH';
        break;
      case SortBy.TOTAL_RUSHING_TOUCHDOWNS:
        return 'TOTAL_RUSHING_TOUCHDOWNS';
        break;
      case SortBy.TOTAL_RUSHING_YARDS:
        return 'TOTAL_RUSHING_YARDS';
        break;
      default:
        return 'LONGEST_RUSH';
    }
  }

  static String getOrder() {
    switch (order) {
      case Order.ASC:
        return 'ASC';
        break;
      case Order.DESC:
        return 'DESC';
        break;
      default:
        return 'ASC';
    }
  }
}
