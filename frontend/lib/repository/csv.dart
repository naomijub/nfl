// ignore: avoid_web_libraries_in_flutter
import 'dart:html';
import 'package:csv/csv.dart';
import 'package:frontend/bloc/info_bloc.dart';

getCsv(String filename, List<Map<String, dynamic>> values) {
  final header = InfoBloc.queryFields;
  final body = values
      .map((e) => InfoBloc.queryFields.map((q) => e[q].toString()).toList())
      .toList();
  List<List<String>> rows = [header] + body;

  String csv = const ListToCsvConverter().convert(rows).trim();

  String encodedFileContents = Uri.encodeComponent(csv);
  print(encodedFileContents);
  new AnchorElement(href: "data:text, $encodedFileContents")
    ..setAttribute("download", "$filename.csv")
    ..click();
}
