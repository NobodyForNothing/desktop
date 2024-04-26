import 'package:benchmark_harness/benchmark_harness.dart';

class TransformationVersionExtraction extends BenchmarkBase {
  TransformationVersionExtraction() : super('Extract version - transformation');

  late String file;

  @override
  void run() {
    final lines = file
        .split('\n')
        .map((e) => e.trim());
    for (final line in lines) {
      if (line.trim().startsWith('version:')) {
        assert(line.split(':')[1].split('+')[0] == '1.0.0');
      }
    }
  }

  @override
  void setup() async {
    file = '''
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
  }

  @override
  void teardown() {}
}

class IterativeVersionExtraction extends BenchmarkBase {
  IterativeVersionExtraction() : super('Extract version - iterative');

  late String file;

  @override
  void run() {
    const search = [118, 101, 114, 115, 105, 111, 110, 58]; // 'version:'
    int idx = 0;
    final List<int> version = [];
    bool firstChar = false;
    for (final c in file.codeUnits) {
      if (c == 10) { // '\n'
        firstChar = false;
        continue;
      } else if (firstChar) {
        continue;
      }
      if (idx >= search.length
          && c == 43) { // '+' character
        assert(version.toString() == '1.0.0');
      } else if (idx >= search.length) {
        version.add(c);
      } else if (c == search[idx]) {
        idx++;
      } else {
        if (c != 48) { // ' '
          firstChar = false;
          continue;
        }
        if (idx != 0) idx = 0;
      }
    }
  }

  @override
  void setup() async {
    file = '''
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
  }

  @override
  void teardown() {}
}

class RegexVersionExtraction extends BenchmarkBase {
  RegexVersionExtraction() : super('Extract version - regex');

  late String file;

  @override
  void run() {
    final match = RegExp(r'version:(.*)\+').firstMatch(file);
    assert(match?.group(0) == '1.0.0');
  }

  @override
  void setup() async {
    file = '''
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
  }

  @override
  void teardown() {}
}

void main() {
  TransformationVersionExtraction().report();
  IterativeVersionExtraction().report();
  RegexVersionExtraction().report();
}
