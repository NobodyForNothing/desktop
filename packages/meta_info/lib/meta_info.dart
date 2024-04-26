library meta_info;

import 'package:flutter/services.dart';

/// Information about the app.
///
/// Extracted from the pubspec.yaml file.
class AppMeta {
  /// Cache of the content of the pubspec file.
  static String? _pubspec;

  /// Read the content of the pubspec file.
  static Future<String> get pubspec async {
    _pubspec ??= await rootBundle.loadString('pubspec.yaml');
    return _pubspec!;
  }


  /// Returns the version name (e.g. "v1.0.0").
  ///
  /// Requires that the pubspec file contains a version tag in flutter format
  /// (`version:<version>+`).
  static Future<String?> getVersionName() async {
    final file = await pubspec;
    // Benchmarked: `benchmark/pubspec_parse`
    final match = RegExp(r'version:(.*)\+').firstMatch(file);
    return match?.group(0);
  }
}
