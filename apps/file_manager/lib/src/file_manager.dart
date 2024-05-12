import 'dart:io';

import 'package:file_entry/widgets/file_list_tile.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;

/// Experimental file manager implementation.
class FileManager extends StatefulWidget {
  /// Create experimental file manager.
  const FileManager({super.key});

  @override
  State<FileManager> createState() => _FileManagerState();
}

class _FileManagerState extends State<FileManager> {
  final _controller = TextEditingController();
  String? _error;

  String _path = '/home/derdilla/';

  String get path => _path;
  set path(String value) {
    _path = value;
    _controller.text = _path;
  }

  Widget _focus(bool autofocus, Widget child) => Focus(
    autofocus: autofocus,
    child: child,
  );

  @override
  Widget build(BuildContext context) {
    final entries = Directory(path).listSync();
    bool first = true;
    return Scaffold(
      appBar: AppBar(
        toolbarHeight: _error == null ? null : 100,
        actions: [
          IconButton(
            onPressed: () {
              setState(() {
                path = p.normalize(p.join(path,'..'));
              });
            },
            icon: const Icon(Icons.arrow_upward)
          ),
        ],
        title: TextField(
          controller: _controller,
          onChanged: (txt) {
            if (File(txt).existsSync()) {
              setState(() {
                _error = "This folder doesn't exist";
              });
            } else {
              setState(() {
                _error = null;
              });
            }
          },
          onSubmitted: (txt) {
            if (_error == null) {setState(() {
              path = txt;
            });}
          },
          decoration: InputDecoration(
            errorText: _error,
          ),
        ),
      ),
      body: ListView(
        children: [
          for (final folder in entries.where((e) => e.statSync()
              .type == FileSystemEntityType.directory))
            _focus(
              () {if (first) {first = false; return true;} return false;}(),
              ListTile(
                leading: Icon(Icons.folder_outlined,
                    color: Theme.of(context).colorScheme.primary),
                title: Text(p.basename(folder.path)),
                onTap: () {
                  setState(() {
                    path = folder.path;
                  });
                },
              ),
            ),
          for (final file in entries.where((e) => e.statSync()
              .type == FileSystemEntityType.file))
            _focus(
              () {if (first) {first = false; return true;} return false;}(),
              FileListTile(
                file: File(file.path),
                options: const [],
              ),
            ),
        ],

      ),
    );
  }

}
