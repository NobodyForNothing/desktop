import 'package:calculator/src/calculator_logic.dart';
import 'package:calculator/src/settings_store.dart';
import 'package:flutter/material.dart';

/// Screen to configure preferences and show version info.
class Settings extends StatelessWidget {
  /// Create screen to configure preferences and show version info.
  const Settings({super.key, required this.eqManager});

  /// Equation manager as used in the calculator.
  ///
  /// Needed to clear history.
  final EquationManager eqManager;

  Widget _buildAppInfo(BuildContext context) => Column(
    mainAxisSize: MainAxisSize.min,
    children: [
      CircleAvatar(
        radius: MediaQuery.of(context).size.width / 6,
        backgroundColor: Colors.transparent,
        child: Image.asset('icon.png', fit: BoxFit.cover,),
      ),
      const Text('Calculator'),
      // TODO
      Text('v1.0.0', style: Theme.of(context).textTheme.labelMedium,)
    ],
  );

  Widget _buildSettings(BuildContext context) => StreamBuilder(
    stream: SettingsStore.stream,
    builder: (context, snapshot) => Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.all(16.0),
          child: Text('Keyboard',
            style: Theme.of(context).textTheme.titleLarge),
        ),
        SwitchListTile(
          title: const Text('Allow native'),
          value: SettingsStore.allowNative,
          onChanged: (value) => SettingsStore.allowNative = value,
        ),
        SwitchListTile(
          title: const Text('Hide math input'),
          value: SettingsStore.hideMath,
          onChanged: (value) => SettingsStore.hideMath = value,
        ),
      ],
    )
  );

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(),
    body: Column(
      children: [
        _buildAppInfo(context),
        _buildSettings(context),
        StreamBuilder(
          stream: eqManager.history,
          builder: (context, snapshot) => TextButton(
            onPressed: eqManager.isHistoryEmpty
                ? null
                : eqManager.clearHistory,
            child: const Text('Clear calculation history'),
          ),
        ),
      ],
    ),
  );

}
