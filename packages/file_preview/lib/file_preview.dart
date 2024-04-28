library file_preview;

import 'dart:io';

import 'package:assets/assets.dart';
import 'package:file_preview/preview_data.dart';
import 'package:flutter/material.dart';

/// A smart visual preview for a file.
class FilePreview extends StatelessWidget {
  /// Create a smart visual preview from a file.
  const FilePreview({super.key,
    required this.file
  });

  /// The disk file to display.
  final File file;

  @override
  Widget build(BuildContext context) => StreamBuilder(
      stream: PreviewData.track(file),
      builder: (context, snapshot) => switch (snapshot.data) {
            null || GenericFile() => Assets.genericFile,
            ImagePreview() => const Icon(Icons.image_outlined),
            VideoPreview() => const Icon(Icons.video_file_outlined),
            DocumentPreview() => const Icon(Icons.description_outlined),
          });
}
