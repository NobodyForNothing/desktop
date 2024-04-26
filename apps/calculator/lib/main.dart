import 'package:calculator/src/calculator.dart';
import 'package:calculator/src/equation_manager.dart';
import 'package:calculator/src/rust/frb_generated.dart';
import 'package:calculator/src/widgets/calc_field.dart';
import 'package:calculator/src/widgets/history.dart';
import 'package:calculator/src/widgets/numpad.dart';
import 'package:flutter/material.dart';

void main() async {
  await RustLib.init();
  runApp(const MyApp());
}

/// App root to provide dependencies and routing.
class MyApp extends StatelessWidget {
  /// Create primary app screen.
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) => MaterialApp(
    debugShowCheckedModeBanner: false,
    title: 'Calculator',
    theme: ThemeData(
      colorScheme: ColorScheme.fromSeed(
        brightness: Brightness.dark,
        seedColor: Colors.amber,
      ),
      useMaterial3: true,
    ),
    home: Calculator(),
  );
}
