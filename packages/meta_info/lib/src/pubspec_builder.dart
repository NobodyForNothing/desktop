import 'package:flutter/material.dart';
import 'package:meta_info/meta_info.dart';

/// Builder with information extracted from the contexts `pubspec.yaml`.
///
/// Requires that `pubspec.yaml` is adds itself in the assets section.
class PubspecBuilder extends StatelessWidget {
  /// Create builder for the information in the contexts `pubspec.yaml`
  const PubspecBuilder({
    super.key,
    required this.builder,
    this.child,
  });

  /// Called with a handle to the information to obtain the child widget.
  ///
  /// During load no pubspec.yaml is available.
  final ValueWidgetBuilder<PubspecParser?> builder;

  /// Data independent widget passed back to the builder.
  ///
  /// May improve performance in situations where a large subtree is required.
  final Widget? child;

  @override
  Widget build(BuildContext context) => FutureBuilder(
        future: DefaultAssetBundle.of(context).loadString('pubspec.yaml'),
        builder: (context, snapshot) {
          if (snapshot.data == null) return builder(context, null, child);
          final parser = PubspecParser(snapshot.data!);
          return builder(context, parser, child);
        },
      );
}
