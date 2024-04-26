import 'dart:io';
import 'package:path/path.dart';

/// Data for constructing a preview.
sealed class PreviewData {
  // TODO: validate structure with benchmarks

  /// Determine which type of file this is construct the appropriate previews.
  ///
  /// Automatically listens for changes in the file and updates the preview
  /// accordingly.
  static Stream<PreviewData> track(File file) async* {
    assert(
        file.statSync().type == FileSystemEntityType.file,
        'Expecting files.'
        ' Consider creating a directory variant or implementing them here.');

    yield GenericFile();

    var data = _analyze(file);
    if (data != null) yield data;

    await for (final event in file.watch(events: FileSystemEvent.modify)) {
      assert(event is FileSystemModifyEvent);
      if ((event as FileSystemModifyEvent).contentChanged) {
        data = _analyze(file);
        if (data != null) yield data;
      }
    }
  }

  static PreviewData? _analyze(File file) =>
      switch (extension(file.path.toLowerCase())) {
        // TODO: content preview and more types
        // - audio
        '.png' || '.jpg' || '.jpeg' => ImagePreview(),
        '.mp4' || '.webm' => VideoPreview(),
        '.pdf' || '.doc' || '.docx' => DocumentPreview(),
        String() => null,
      };
}

/// File without further information.
class GenericFile extends PreviewData {}

/// Image preview.
class ImagePreview extends PreviewData {}

/// Video preview.
class VideoPreview extends PreviewData {}

/// Document preview.
class DocumentPreview extends PreviewData {}

// TODO_ test
