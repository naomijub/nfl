import 'package:flutter/material.dart';
import 'package:frontend/Views/nfl_scaffold.dart';
import 'package:frontend/Views/players_table.dart';
import 'package:frontend/bloc/info_bloc.dart';
import 'package:frontend/repository/csv.dart';
import 'package:frontend/repository/http.dart';

class ListAllPlayers extends StatefulWidget {
  ListAllPlayers({Key key}) : super(key: key);

  @override
  _ListAllPlayersState createState() => _ListAllPlayersState();
}

class _ListAllPlayersState extends State<ListAllPlayers> {
  Future<Map<String, Map<String, List<Map<String, dynamic>>>>> queryResponse;
  List<Map<String, dynamic>> info = List();

  @override
  void initState() {
    super.initState();
    queryResponse = queryAllPlayers(InfoBloc.getQueryFieldsStr());
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: Container(
        child: NflScaffold(
          title: 'All Players',
          button: FloatingActionButton(
            child: Icon(
              Icons.cloud_download,
            ),
            onPressed: () {
              getCsv('listAllPlayers', info);
            },
          ),
          body: FutureBuilder<
              Map<String, Map<String, List<Map<String, dynamic>>>>>(
            future: queryResponse,
            builder: (context, snapshot) {
              if (snapshot.hasData) {
                info = snapshot.data['data']['listPlayers'];
                return Container(
                  padding: EdgeInsets.only(top: 16),
                  child: PlayersTable(values: info),
                );
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
      ),
    );
  }
}
