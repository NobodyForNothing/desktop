import 'dart:io';

import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;

/// A screen with detailed info about the file.
class FileInfo extends StatelessWidget {
  /// Create a screen with detailed info about the file.
  const FileInfo({super.key, required this.file});

  /// The disk file to display.
  final File file;

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      title: Text(p.basename(file.path)),
    ),
    body: ListView(
      children: [
        ListTile(
          title: const Text('Size:'),
          subtitle: Text('${file.statSync().size} Bytes'),
          // TODO: check if this is bytes and format for readability
        ),
        /*ListTile(
          title: Text('Created: ${file.}'),
        ),*/
        ListTile(
          title: const Text('Last accessed'),
          subtitle: Text(file.lastAccessedSync().toString()),
        ),
        ListTile(
          title: const Text('Last modified'),
          subtitle: Text(file.lastModifiedSync().toString()),
        ),
        // TODO:
        // - create
        // - permissions
        // - editing
        // - programms to open
        // - preview
      ],
    ),
  );
}
