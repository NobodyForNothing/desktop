import 'dart:io';

import 'package:flutter_test/flutter_test.dart';

import 'package:file_preview/file_preview.dart';

void main() {
  testWidgets('should initialize', (tester) async {
    await tester.pumpWidget(FilePreview(file: File('samples/text-file.txt')));
    await tester.pumpAndSettle();
  });
  // TODO
}
