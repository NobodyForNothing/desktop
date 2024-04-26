import 'package:calculator/src/equation_manager.dart';
import 'package:calculator/src/widgets/calc_field.dart';
import 'package:calculator/src/widgets/history.dart';
import 'package:calculator/src/widgets/numpad.dart';
import 'package:flutter/material.dart';

class Calculator extends StatefulWidget {
  @override
  State<Calculator> createState() => _CalculatorState();
}

class _CalculatorState extends State<Calculator> {
  final EquationManager _manager = EquationManager();

  @override
  void initState() {
    super.initState();
    _manager.setUp();
  }

  @override
  void dispose() {
    _manager.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      title: const Text('Calculator'),
      centerTitle: true,
      actions: [
        IconButton(
          icon: const Icon(Icons.settings),
          onPressed: () {
            // TODO:
            // - keyboard type
            // - clear history
            // - app info
          },
        ),
      ],
    ),
    body: Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        History(equationManager: _manager),
        CalcField(
          equationManager: _manager,
        ),
        Numpad(
          onEntered: (v) => _manager.inputController.text += v,
          onSubmit: () => _manager.submit(_manager.inputController.text),
        ),
      ],
    ),
  );
}
