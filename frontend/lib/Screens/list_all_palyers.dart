import 'package:flutter/material.dart';
import 'package:frontend/Views/nfl_scaffold.dart';
import 'package:frontend/bloc/info_bloc.dart';
import 'package:frontend/repository/http.dart';

class ListAllPlayers extends StatefulWidget {
  ListAllPlayers({Key key}) : super(key: key);

  @override
  _ListAllPlayersState createState() => _ListAllPlayersState();
}

class _ListAllPlayersState extends State<ListAllPlayers> {
  Future<String> queryResponse;

  @override
  void initState() {
    super.initState();
    queryResponse = queryAllPlayers(InfoBloc.getQueryFieldsStr());
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      child: NflScaffold(
        title: 'All Players',
        body: FutureBuilder<String>(
          future: queryResponse,
          builder: (context, snapshot) {
            if (snapshot.hasData) {
              return Text(snapshot.data.toString());
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
