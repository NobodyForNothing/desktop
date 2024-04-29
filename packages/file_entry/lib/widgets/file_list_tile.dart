import 'dart:io';

import 'package:file_entry/widgets/file_info.dart';
import 'package:file_preview/file_preview.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart';

/// A responsive and adaptive material design list tile for a single file.
class FileListTile extends StatelessWidget {
  /// Create A responsive and adaptive material list tile for a single file.
  const FileListTile({super.key, required this.file, required this.options});

  /// The disk file to display.
  final File file;

  /// Additional options for the context menu
  final List<PopupMenuEntry> options;

  // TODO: file selection (external mixin ?)
  // TODO: folder support
  // TODO: display app size and other info if width supports it

  @override
  Widget build(BuildContext context) {
    assert(file.statSync().type == FileSystemEntityType.file);
    return ListTile(
      leading: FilePreview(file: file),
      title: Text(basename(file.path)),
      onTap: () {
        // TODO
      },
      trailing: PopupMenuButton(
        icon: const Icon(Icons.more_vert),
        itemBuilder: (BuildContext context) => <PopupMenuEntry>[
          const PopupMenuDivider(),
          PopupMenuItem(
            child: const Text('Properties'),
            onTap: () {
              showDialog(
                  context: context, builder: (context) => FileInfo(file: file));
            },
          )
        ],
      ),
    );
  }
}
