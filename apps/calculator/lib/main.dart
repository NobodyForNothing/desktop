import 'package:app_base/app_base.dart';
import 'package:calculator/src/calculator.dart';
import 'package:calculator/src/rust/frb_generated.dart';

void main() async {
  await RustLib.init();
  runDerdillaApp(
    home: const Calculator(),
    title: 'Calculator',
  );
  // TODO: fix app version
}
