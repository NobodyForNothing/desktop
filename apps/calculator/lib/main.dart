// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

import 'dart:math';

import 'package:calculator/calc_field.dart';
import 'package:calculator/numpad.dart';
import 'package:calculator/src/rust/frb_generated.dart';
import 'package:flutter/material.dart';
void main() async {
  await RustLib.init();
  runApp(const MyApp());
}
class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    // Dimensions should be a square that doesn't overflow in half height or
    // width.
    final screenSize = MediaQuery.of(context).size;
    final sideLength = min(screenSize.height / 2, screenSize.width);
    return MaterialApp(
      title: 'Calculator',
      theme: ThemeData(
        colorScheme: const ColorScheme.dark(),
        useMaterial3: true,
      ),
      home: Scaffold(
        body: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Expanded(child: const CalcField()),
            Numpad(
              onEntered: (v) {
                print(v);
              },
            ),
          ],
        ),
      ),
    );
  }
}
