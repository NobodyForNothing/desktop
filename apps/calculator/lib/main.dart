import 'package:calculator/equation_manager.dart';
import 'package:calculator/src/rust/frb_generated.dart';
import 'package:calculator/src/widgets/calc_field.dart';
import 'package:calculator/src/widgets/history.dart';
import 'package:calculator/src/widgets/numpad.dart';
import 'package:flutter/material.dart';

void main() async {
  await RustLib.init();
  runApp(const MyApp());
}

/// Primary app screen.
///
/// Composes overall layout and hosts state management.
class MyApp extends StatefulWidget {
  /// Create primary app screen.
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
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
  Widget build(BuildContext context) => MaterialApp(
        debugShowCheckedModeBanner: false,
        title: 'Calculator',
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(
            brightness: Brightness.dark,
            seedColor: Colors.amber,
          ),
          useMaterial3: true,
        ),
        home: Scaffold(
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
        ),
      );
}
