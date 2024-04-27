import 'package:flutter_test/flutter_test.dart';

import 'package:meta_info/meta_info.dart';

/// Sample pubspec file.
///
/// name: calculator
/// description: "Simple and usable calculator"
/// version: 1.0.0
const samplePubspec = '''
name: calculator
description: "Simple and usable calculator"
publish_to: 'none'

version: 1.0.0+1

environment:
  sdk: '>=3.2.0 <4.0.0'

dependencies:
  flutter:
    sdk: flutter
  flutter_native_splash: ^2.3.11
  flutter_rust_bridge: ^2.0.0-dev.32
  rust_lib_calculator:
    path: rust_builder
  url_launcher: ^6.2.6

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^3.0.0
  integration_test:
    sdk: flutter

flutter:
  uses-material-design: true
  assets:
    - icon.png

''';

void main() {
  test('should extract version', () async {
    const parser = PubspecParser(samplePubspec);
    final version = await parser.getVersionName();
    expect(version, equals('1.0.0'));
  });
}
