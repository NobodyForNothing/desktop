import 'dart:async';

import 'package:calculator/src/rust/api/calc.dart';
import 'package:flutter/scheduler.dart';
import 'package:flutter/widgets.dart';

/// Calculator logic and state management.
class EquationManager {
  final List<(String, String?)> _lastHistory = [];

  final _error = StreamController<String?>();
  final _history = StreamController<List<(String, String?)>>();

  /// Text of the current input field.
  final TextEditingController inputController = TextEditingController();

  /// Key for animated history.
  final keys = GlobalKey<AnimatedListState>();

  /// Controller to ensure that the history always shows the latest results.
  final historyScrollController = ScrollController();

  /// Focus node for the equation input
  final FocusNode inputFocus = FocusNode();
  
  /// Previously submitted equations and their results.
  Stream<List<(String, String?)>> get history => _history.stream;

  /// Errors during the last calculation.
  Stream<String?> get errors => _error.stream;

  /// Calculate the result of an equation.
  Future<void> submit(String text) async {
    _error.sink.add(null);
    final val = await interpret(equation: text);
    if (val != null) {
      inputController.clear();
      inputFocus.requestFocus();
      _addToHistory(text, val);
    } else {
      _error.sink.add('Unable to calculate result');
    }
  }

  /// Sets up all managed widgets.
  ///
  /// Should be called once all widgets are inserted into the tree.
  void setUp() {
    inputFocus.requestFocus();
  }

  /// Discards any resources used by the object.
  void dispose() {
    inputController.dispose();
    inputFocus.dispose();
    historyScrollController.dispose();
  }

  void _addToHistory(String equation, String? result) {
    _lastHistory.add((equation, result));
    _history.sink.add(_lastHistory);
    keys.currentState?.insertItem(
      _lastHistory.length - 1,
      duration: const Duration(milliseconds: 500)
    );
    SchedulerBinding.instance.addPostFrameCallback((_) =>
      historyScrollController.animateTo(
        historyScrollController.position.maxScrollExtent,
        duration: const Duration(milliseconds: 500),
        curve: Curves.ease
      )
    );
  }
}
