import 'package:app_base/src/derdilla_theme.dart';
import 'package:flutter/material.dart';

/// The basis for a derdilla application.
/// 
/// Provides a styled variant material design. 
class DerdillaApp extends StatelessWidget {
  /// Create a derdilla app.
  const DerdillaApp({super.key,
    required this.title,
    required this.home,
  });

  /// This value is passed unmodified to [WidgetsApp.title].
  final String title;

  /// Primary widget
  final Widget home;

  @override
  Widget build(BuildContext context) => MaterialApp(
    debugShowCheckedModeBanner: false,
    theme: derdillaTheme,
    title: title,
    home: home,
  );
  
}
