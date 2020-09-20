import 'package:flutter/material.dart';
import 'package:frontend/Views/nfl_scaffold.dart';
import 'package:frontend/bloc/info_bloc.dart';
import 'package:frontend/repository/http.dart';

class SortedPlayers extends StatefulWidget {
  SortedPlayers({Key key}) : super(key: key);

  @override
  _SortedPlayersState createState() => _SortedPlayersState();
}

class _SortedPlayersState extends State<SortedPlayers> {
  Future<Map<String, Map<String, List<Map<String, dynamic>>>>> queryResponse;

  @override
  void initState() {
    super.initState();
    queryResponse = querySortPlayers(InfoBloc.getSorting(), InfoBloc.getOrder(),
        InfoBloc.getQueryFieldsStr());
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      child: NflScaffold(
        title: 'Sorted Players',
        body:
            FutureBuilder<Map<String, Map<String, List<Map<String, dynamic>>>>>(
          future: queryResponse,
          builder: (context, snapshot) {
            if (snapshot.hasData) {
              return Text(snapshot.data['data']['sortPlayers'].toString());
            } else if (snapshot.connectionState == ConnectionState.waiting) {
              return Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: <Widget>[
                    SizedBox(
                      child: CircularProgressIndicator(
                        strokeWidth: 10,
                      ),
                      height: 200.0,
                      width: 200.0,
                    ),
                    Padding(
                      padding: const EdgeInsets.fromLTRB(0, 10, 0, 0),
                      child: Center(
                          child: Text(
                        'Loading',
                        textScaleFactor: 14,
                      )),
                    ),
                  ],
                ),
              );
            } else if (snapshot.hasError) {
              return Text("${snapshot.error}");
            }

            return CircularProgressIndicator();
          },
        ),
      ),
    );
  }
}
