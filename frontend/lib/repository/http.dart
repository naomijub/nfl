import 'dart:convert';

import 'package:frontend/bloc/info_bloc.dart';
import 'package:http/http.dart' as http;

Future<String> queryNflRushing() async {
  final response = await http.post(InfoBloc.uri,
      headers: {
        'Accept': '*/*',
        'Access-Control-Allow-Origin': '*',
        'content-type': 'application/json; charset=utf-8'
      },
      body: jsonEncode({
        'query':
            'query { playersByName(perPage: ${InfoBloc.perPage}, page: ${InfoBloc.page}, pattern: "${InfoBloc.name}" ) { name }}'
      }));
  return response.body;
}
