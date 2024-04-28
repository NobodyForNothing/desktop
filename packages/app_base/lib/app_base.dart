library app_base;

import 'dart:io';

import 'package:app_base/src/derdilla_app.dart';
import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';

/// Run a app in derdilla style without title bar.
void runDerdillaApp({
  required String title,
  required Widget home,
}) async {
  WidgetsFlutterBinding.ensureInitialized();
  if (Platform.isLinux || Platform.isMacOS || Platform.isWindows) {
    await windowManager.ensureInitialized();
    final WindowOptions windowOptions = WindowOptions(
      backgroundColor: Colors.transparent,
      skipTaskbar: false,
      titleBarStyle: TitleBarStyle.hidden,
      title: title,
    );
    // TODO: custom app bar with closing options
    await windowManager.waitUntilReadyToShow(windowOptions, () async {
      await windowManager.show();
    });
  }

  runApp(DerdillaApp(
    title: title,
    home: home,
  ));
}
