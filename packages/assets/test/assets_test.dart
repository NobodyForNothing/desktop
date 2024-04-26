import 'package:flutter_test/flutter_test.dart';

import 'package:assets/assets.dart';

void main() {
  testWidgets('should load all assets', (tester) async {
    await tester.pumpWidget(Assets.genericFile);
    await tester.pumpAndSettle();
  });
}
