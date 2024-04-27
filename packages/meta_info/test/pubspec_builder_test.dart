import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter/widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:meta_info/meta_info.dart';

import 'pubspec_parser.dart';

class _MockBundle extends AssetBundle {
  @override
  Future<ByteData> load(String key) async {
    if (key != 'pubspec.yaml') {
      throw UnimplementedError('Could be more generic when used in more tests');
    }
    final string = utf8.encode(samplePubspec);

    final data = ByteData(string.length);
    for (int i = 0; i < string.length; i++) {
      data.setInt8(i, string[i]);
    }
    return data;
  }
}

void main() {
  testWidgets('passes child element', (tester) async {
    const originalChild = SizedBox(width: 123);
    bool called = false;
    await tester.pumpWidget(PubspecBuilder(
      child: originalChild,
      builder: (_, __, passedChild) {
        expect(passedChild, originalChild);
        called = true;
        return passedChild!;
    }));
    expect(called, isTrue);
  });
  testWidgets('returns contexts pubspec', (tester) async {
    Future<String?>? version;
    await tester.pumpWidget(DefaultAssetBundle(
      bundle: _MockBundle(),
      child: PubspecBuilder(
        builder: (_, PubspecParser? value, __) {
          if (value == null) return const SizedBox();
          version = value.getVersionName();
          return const SizedBox();
        },
      ),
    ));
    await tester.pumpAndSettle();
    expect(version, isNotNull);
    final v = await version;
    expect(v, equals('1.0.0'));
  });
}
