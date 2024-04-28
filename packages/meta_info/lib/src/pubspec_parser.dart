
import 'package:meta_info/src/pubspec_builder.dart';

/// Lazy parser for information from the pubspec.yaml file.
///
/// Consider using [PubspecBuilder] to obtain a instance.
class PubspecParser {
  /// Create a new parser for metadata stored in the pubspec file.
  ///
  /// The pubspec file is expected to be a flutter style pubspec file from the
  /// asset bundle. Example obtaining:
  /// `DefaultAssetBundle.of(context).loadString('pubspec.yaml')`
  const PubspecParser(this._pubspec);

  /// Cache of the content of the pubspec file.
  final String _pubspec;

  /// Returns the version name (e.g. "1.0.0").
  ///
  /// Requires that the pubspec contains a version key in with a value formated
  /// like `version: <version>+<buildNum>`.
  String? getVersionName() {
    // Benchmarked: `benchmark/pubspec_parse.dart`
    final match = RegExp(r'version\s*:\s*(.*)\+').firstMatch(_pubspec);
    return match?.group(1);
  }
}
