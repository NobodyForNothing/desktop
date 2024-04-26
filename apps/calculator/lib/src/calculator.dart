import 'package:calculator/src/calculator_logic.dart';
import 'package:calculator/src/settings.dart';
import 'package:calculator/src/widgets/calc_field.dart';
import 'package:calculator/src/widgets/history.dart';
import 'package:calculator/src/widgets/numpad.dart';
import 'package:flutter/material.dart';

/// Primary calculator screen.
///
/// Hosts history, keyboard and input and state management.
class Calculator extends StatefulWidget {
  /// Create primary calculator screen.
  const Calculator({super.key});

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
          onPressed: () => Navigator.push(context, MaterialPageRoute(
            builder: (_) => Settings(eqManager: _manager,),
          )),
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
