// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

import 'package:calculator/calc_field.dart';
import 'package:calculator/src/rust/frb_generated.dart';
import 'package:flutter/material.dart';
void main() async {
  await RustLib.init();
  runApp(const MyApp());
}
class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) => MaterialApp(
    title: 'Calculator',
    theme: ThemeData(
      colorScheme: const ColorScheme.dark(),
      useMaterial3: true,
    ),
    home: const Scaffold(
      body: SizedBox(
        child: CalcField()
      ),
    ),
  );
}
