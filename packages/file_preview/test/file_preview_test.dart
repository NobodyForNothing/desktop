import 'dart:io';

import 'package:file_preview/file_preview.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('should initialize', (tester) async {
    await tester.pumpWidget(FilePreview(file: File('samples/text-file.txt')));
    await tester.pumpAndSettle();
  });
  // TODO
}
