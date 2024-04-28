import 'package:flutter/material.dart';

/// Theme of a derdilla-style application.
class DerdillaTheme extends ThemeData {
  /// Create the theme of a derdilla-style application.
  factory DerdillaTheme() => ThemeData(
    colorScheme: ColorScheme.fromSeed(
      brightness: Brightness.dark,
      seedColor: Colors.amber,
    ),
    useMaterial3: true,
  ) as DerdillaTheme;
}
