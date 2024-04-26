import 'package:calculator/src/equation_manager.dart';
import 'package:flutter/material.dart';

/// Field that allows inputting new calculations.
class CalcField extends StatelessWidget {
  /// Create a text field for calculations.
  const CalcField({
    super.key,
    required this.equationManager,
  });

  /// Controls state updates.
  final EquationManager equationManager;

  @override
  Widget build(BuildContext context) => Padding(
        padding: const EdgeInsets.all(8.0),
        child: StreamBuilder<String?>(
            stream: equationManager.errors,
            builder: (context, snapshot) => TextField(
                  controller: equationManager.inputController,
                  focusNode: equationManager.inputFocus,
                  keyboardType: TextInputType.none,
                  decoration: InputDecoration(
                      errorText: snapshot.data,
                      suffixIcon: IconButton(
                        onPressed: equationManager.inputController.clear,
                        icon: const Icon(Icons.clear),
                      ),
                      border: const OutlineInputBorder(),
                      hintText: '2 + x = 4'),
                  onSubmitted: equationManager.submit,
                )),
      );
}
