import 'dart:async';
import 'dart:collection';

import 'package:calculator/src/rust/api/calc.dart';
import 'package:flutter/scheduler.dart';
import 'package:flutter/widgets.dart';

class EquationManager {
  final List<(String, String?)> _lastHistory = [];

  final _error = StreamController<String?>();
  final _history = StreamController<List<(String, String?)>>();

  /// Key for animated history.
  final keys = GlobalKey<AnimatedListState>();

  final historyScrollController = ScrollController();
  
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
      keys.currentState?.insertItem(
          _lastHistory.length - 1,
          duration: const Duration(milliseconds: 500)
      );
      SchedulerBinding.instance.addPostFrameCallback((_) =>
        historyScrollController.animateTo(
          historyScrollController.position.maxScrollExtent,
          duration: const Duration(milliseconds: 500),
          curve: Curves.ease
      ));
    } else {
      _error.sink.add('Unable to calculate result');
    }
  }
}
