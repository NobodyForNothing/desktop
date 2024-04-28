import 'dart:io';

import 'package:file_entry/widgets/file_list_tile.dart';
import 'package:flutter/material.dart';

class FileManager extends StatefulWidget {
  @override
  State<FileManager> createState() => _FileManagerState();
}

class _FileManagerState extends State<FileManager> {
  String path = '/home/derdilla/';
  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      title: TextField(
        decoration: InputDecoration(
          hintText: path,
        ),
      ),
    ),
    body: ListView(
      children: [
        for (final file in Directory(path)
            .listSync()
            .where((e) => e.statSync().type == FileSystemEntityType.file))
          FileListTile(
            file: File(file.path),
            options: const [],
          )
      ],

    ),
  );

}
