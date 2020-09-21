import 'package:flutter/material.dart';
import 'package:frontend/bloc/info_bloc.dart';

class PlayersTable extends StatelessWidget {
  const PlayersTable({Key key, this.values}) : super(key: key);
  final List<Map<String, dynamic>> values;

  @override
  Widget build(BuildContext context) {
    return ListView(
      children: [
        Table(
            border: TableBorder.all(),
            children: List.from([
              TableRow(
                children: InfoBloc.queryFields
                    .map((e) => Text(
                          e,
                          textAlign: TextAlign.center,
                          style: TextStyle(fontSize: 18),
                        ))
                    .toList(),
              )
            ])
              ..addAll(rows())),
      ],
    );
  }

  List<TableRow> rows() {
    return values
        .map(
          (e) => TableRow(
              children: InfoBloc.queryFields
                  .map(
                    (q) => Text(
                      e[q].toString(),
                      textAlign: TextAlign.center,
                    ),
                  )
                  .toList()),
        )
        .toList();
  }
}
