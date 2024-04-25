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
class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late final TextEditingController _controler;


  @override
  void initState() {
    super.initState();
    _controler = TextEditingController();
  }


  @override
  void dispose() {
    _controler.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => MaterialApp(
      title: 'Calculator',
      theme: ThemeData(
        colorScheme: const ColorScheme.dark(),
        useMaterial3: true,
      ),
      home: Scaffold(
        body: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Expanded(
              child: CalcField(
              controller: _controler,
              )
            ),
            Numpad(
              onEntered: (v) => _controler.text += v,
            ),
          ],
        ),
      ),
    );
}
