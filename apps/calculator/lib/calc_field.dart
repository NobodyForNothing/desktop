import 'package:calculator/src/rust/api/simple.dart';
import 'package:flutter/material.dart';

/// Field that allows inputting new calculations and shows a immutable history.
class CalcField extends StatefulWidget {
  /// Create a text field for calculations.
  const CalcField({
    super.key,
    this.controller,
  });

  /// Controls the calculation input being edited.
  final TextEditingController? controller;

  @override
  State<CalcField> createState() => _CalcFieldState();
}

class _CalcFieldState extends State<CalcField> {
  late final TextEditingController _controller;
  late final FocusNode _focus;

  final List<(String, String?)> _history = [];
  String? _error;

  @override
  void initState() {
    super.initState();
    _controller = widget.controller ?? TextEditingController();
    _focus = FocusNode();
    _focus.requestFocus();
  }

  @override
  void dispose() {
    if (widget.controller == null) _controller.dispose();
    _focus.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Column(
    mainAxisSize: MainAxisSize.max,
    children: [
      const Expanded(child: SizedBox.shrink()),
      for (final e in _history)
        ListTile(
          title: Text(e.$1),
          onTap: () {
            // TODO: add result to field
            _controller.text += '$e ';
          },
          trailing: e.$2 == null ? null : Text(e.$2!),
        ),
      Padding(
        padding: const EdgeInsets.all(8.0),
        child: TextField(
          controller: _controller,
          focusNode: _focus,
          decoration: InputDecoration(
            errorText: _error,
            //labelText: 'Equation input'
            suffixIcon: IconButton(
              onPressed: () => setState(() {
                _controller.clear();
              }),
              icon: const Icon(Icons.clear),
            ),
            border: const OutlineInputBorder(),
            hintText: '2 + x = 4'),
          onSubmitted: (text) async {
            setState(() {
              _error = null;
              _history.add((text, null));
            });
            final val = await interpret(equation: text);
            if (val != null) {
              setState(() {
                _history.removeLast();
                _history.add((text, val));
                _controller.clear();
                _focus.requestFocus();
              });
            } else {
              setState(() {
                _history.removeLast();
                _error = 'Unable to calculate result';
                _focus.requestFocus();
              });
            }

          },
        ),
      ),
    ],
  );
}
