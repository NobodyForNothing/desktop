import 'package:calculator/src/calculator_logic.dart';
import 'package:flutter/material.dart';

/// History of calculations.
class History extends StatelessWidget {
  /// Create the calculation history.
  const History({
    super.key,
    required this.equationManager,
  });

  /// Controls state updates.
  final EquationManager equationManager;

  // TODO: consider rendering as cards

  @override
  Widget build(BuildContext context) => Flexible(
        fit: FlexFit.tight,
        child: StreamBuilder(
          stream: equationManager.history,
          builder: (context, snapshot) => SelectionArea(
            child: AnimatedList(
              key: equationManager.keys,
              controller: equationManager.historyScrollController,
              initialItemCount: snapshot.data?.length ?? 0,
              itemBuilder: (context, idx, animation) {
                if ((snapshot.data?.length ?? 0) <= idx) {
                  return const SizedBox.shrink();
                }
                // TODO: split up
                return SlideTransition(
                  position: animation.drive(Tween<Offset>(
                          begin: const Offset(.9, .0), end: const Offset(.0, .0))
                      .chain(CurveTween(curve: Curves.ease))),
                  child: FadeTransition(
                    opacity: animation.drive(Tween(begin: .0, end: 1.0)
                        .chain(CurveTween(curve: Curves.ease))),
                    child: ListTile(
                      title: Text(snapshot.data![idx].$1),
                      onTap: () {
                        equationManager.inputController.text +=
                            ' ${snapshot.data![idx].$2 ?? snapshot.data![idx].$1} ';
                      },
                      trailing: snapshot.data![idx].$2 == null
                          ? null
                          : Text(snapshot.data![idx].$2!),
                    ),
                  ),
                );
              },
            ),
          ),
        ),
      );
}
