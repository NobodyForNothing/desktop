import 'dart:math';

import 'package:flutter/material.dart';

/// On screen numpad like field to enter values.
class Numpad extends StatelessWidget {
  /// Create on screen numpad like field to enter values.
  const Numpad({super.key, required this.onEntered});
  
  /// Gets called for every new character/symbol entered.
  /// 
  /// Multi-character symbols may contain an unclosed opening bracket (`sqrt(`).
  final Function(String entered) onEntered;

  Widget _sameCharacterButton(String character, BuildContext context) => Padding(
    padding: const EdgeInsets.all(3),
    child: TextButton(
      onPressed: () => onEntered(character),
      child: Text(character),
    ),
  );

  @override
  Widget build(BuildContext context) => Center(
    child: Column(
      mainAxisSize: MainAxisSize.max,
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        Row( // +, -, *, /
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _sameCharacterButton('+', context),
            _sameCharacterButton('-', context),
            _sameCharacterButton('*', context),
            _sameCharacterButton('/', context),
          ],
        ),
        Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _sameCharacterButton('7', context),
            _sameCharacterButton('8', context),
            _sameCharacterButton('9', context),
            _sameCharacterButton('+', context),
          ],
        ),
        Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _sameCharacterButton('4', context),
            _sameCharacterButton('5', context),
            _sameCharacterButton('6', context),
            _sameCharacterButton('+', context),
          ],
        ),
        Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _sameCharacterButton('1', context),
            _sameCharacterButton('2', context),
            _sameCharacterButton('3', context),
            _sameCharacterButton('+', context),
          ],
        )
      ],
    ),
  );
  
}
