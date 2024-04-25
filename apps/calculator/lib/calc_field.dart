import 'package:calculator/equation_manager.dart';
import 'package:calculator/src/rust/api/calc.dart';
import 'package:flutter/material.dart';

/// Field that allows inputting new calculations and shows a immutable history.
class CalcField extends StatefulWidget {
  /// Create a text field for calculations.
  const CalcField({super.key,
    this.controller,
    required this.equationManager,
  });

  /// Controls the calculation input being edited.
  final TextEditingController? controller;

  /// Controls state updates.
  final EquationManager equationManager;

  @override
  State<CalcField> createState() => _CalcFieldState();
}

class _CalcFieldState extends State<CalcField> {
  late final TextEditingController _controller;
  late final FocusNode _focus;

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
    mainAxisSize: MainAxisSize.min,
    children: [
      Flexible(
        child: StreamBuilder(
          stream: widget.equationManager.history,
          builder: (context, snapshot) => AnimatedList(
            key: widget.equationManager.keys,
            controller: widget.equationManager.historyScrollController,
            initialItemCount: snapshot.data?.length ?? 0,
            itemBuilder: (context, idx, animation) => SlideTransition(
              position: animation.drive(
                  Tween<Offset>(
                    begin: const Offset(.9, .0),
                    end: const Offset(.0, .0)
                  ).chain(CurveTween(curve: Curves.ease))),
              child: FadeTransition(
                opacity: animation
                    .drive(Tween(begin: .0, end:  1.0)
                    .chain(CurveTween(curve: Curves.ease))),
                child: ListTile(
                  title: Text(snapshot.data![idx].$1),
                  onTap: () {
                    _controller.text += ' ${snapshot.data![idx].$2
                        ?? snapshot.data![idx].$1 } ';
                  },
                  trailing: snapshot.data![idx].$2 == null
                      ? null
                      : Text(snapshot.data![idx].$2!),
                ),
              ),
            ),
          )
        ),
      ),
      Padding(
        padding: const EdgeInsets.all(8.0),
        child: StreamBuilder<String?>(
          stream: widget.equationManager.errors,
          builder: (context, snapshot) => TextField(
            controller: _controller,
            focusNode: _focus,
            decoration: InputDecoration(
              errorText: snapshot.data,
              //labelText: 'Equation input'
              suffixIcon: IconButton(
                onPressed: () => setState(() {
                  _controller.clear();
                }),
                icon: const Icon(Icons.clear),
              ),
              border: const OutlineInputBorder(),
              hintText: '2 + x = 4'),
            onSubmitted: widget.equationManager.submit,
          )
        ),
      ),
    ],
  );

 // TODO field clearing and focus
}
