import 'dart:async';
import 'dart:collection';

import 'package:calculator/src/rust/api/calc.dart';

class EquationManager {
  final List<(String, String?)> _lastHistory = [];

  final _error = StreamController<String?>();
  final _history = StreamController<List<(String, String?)>>();
  
  /// Previously submitted equations and their results.
  Stream<List<(String, String?)>> get history => _history.stream;
  
  Stream<String?> get errors => _error.stream;

  /// Calculate the result of an equation.
  Future<void> submit(String text) async {
    _error.sink.add(null);
    final val = await interpret(equation: text);
    if (val != null) {
      _lastHistory.add((text, val));
      _history.sink.add(_lastHistory);
    } else {
      _error.sink.add('Unable to calculate result');
    }
  }
}