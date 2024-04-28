import 'package:calculator/src/calculator_logic.dart';
import 'package:calculator/src/settings_store.dart';
import 'package:flutter/material.dart';
import 'package:meta_info/meta_info.dart';
import 'package:url_launcher/url_launcher.dart';

/// Screen to configure preferences and show version info.
class Settings extends StatelessWidget {
  /// Create screen to configure preferences and show version info.
  const Settings({super.key, required this.eqManager});

  /// Equation manager as used in the calculator.
  ///
  /// Needed to clear history.
  final EquationManager eqManager;

  // TODO: open repository
  Widget _buildAppInfo(BuildContext context) => GestureDetector(
    onTap: () async {
      final url = Uri.parse('https://github.com/NobodyForNothing/desktop/');
      if (await launchUrl(url) && context.mounted) {
        // TODO: pretty snackbar across all apps
        ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text("Couldn't open url")),
        );
      }
    },
    child: Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        CircleAvatar(
          radius: MediaQuery.of(context).size.width / 6,
          backgroundColor: Colors.transparent,
          child: Image.asset('icon.png', fit: BoxFit.cover,),
        ),
        const Text('Calculator'),
        // TODO
        PubspecBuilder(
          builder: (BuildContext context, PubspecParser? value, _) {
            if (value == null) return const SizedBox.shrink();
            return Text(value.getVersionName() ?? 'unknown version',
              style: Theme.of(context).textTheme.labelMedium,);
          },

        )

      ],
    ),
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
