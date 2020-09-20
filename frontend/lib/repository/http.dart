import 'dart:convert';

import 'package:frontend/bloc/info_bloc.dart';
import 'package:http/http.dart' as http;

Future<String> queryPlayersByName(String queryParameters) async {
  final response = await http.post(InfoBloc.uri,
      headers: {
        'Accept': '*/*',
        'Access-Control-Allow-Origin': '*',
        'content-type': 'application/json; charset=utf-8'
      },
      body: jsonEncode({
        'query':
            'query { playersByName(perPage: ${InfoBloc.perPage}, page: ${InfoBloc.page}, pattern: "${InfoBloc.name}" ) { $queryParameters }}'
      }));
  return response.body;
}

Future<String> queryAllPlayers(String queryParameters) async {
  final response = await http.post(InfoBloc.uri,
      headers: {
        'Accept': '*/*',
        'Access-Control-Allow-Origin': '*',
        'content-type': 'application/json; charset=utf-8'
      },
      body: jsonEncode({
        'query':
            'query { listPlayers(perPage: ${InfoBloc.perPage}, page: ${InfoBloc.page}) { $queryParameters }}'
      }));
  return response.body;
}

Future<String> querySortPlayers(
    String sorting, String order, String queryParameters) async {
  final response = await http.post(InfoBloc.uri,
      headers: {
        'Accept': '*/*',
        'Access-Control-Allow-Origin': '*',
        'content-type': 'application/json; charset=utf-8'
      },
      body: jsonEncode({
        'query':
            'query { sortPlayers(perPage: ${InfoBloc.perPage}, page: ${InfoBloc.page}, sortBy: $sorting, order: $order ) { $queryParameters }}'
      }));
  return response.body;
}
