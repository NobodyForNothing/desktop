import 'package:flutter/material.dart';

/// On screen numpad like field to enter values.
class Numpad extends StatelessWidget {
  /// Create on screen numpad like field to enter values.
  const Numpad({super.key, 
    required this.onEntered, 
    required this.onSubmit,
  });
  
  /// Gets called for every new character/symbol entered.
  /// 
  /// Multi-character symbols may contain an unclosed opening bracket (`sqrt(`).
  final Function(String entered) onEntered;
  
  /// Called when the solve button is pressed. 
  final Function() onSubmit;

  Widget _sameCharacterButton(String character) => _characterButton(
    character,
    () => onEntered(character),
  );
  
  Widget _characterButton(String display, void Function() onPressed) =>
    _buttonWrapper(TextButton(
      onPressed: onPressed,
      child: Text(display),
    ));
  
  Widget _buttonWrapper(Widget child) => Padding(
    padding: const EdgeInsets.all(3),
    child: child,
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
            _sameCharacterButton('+'),
            _sameCharacterButton('-'),
            _sameCharacterButton('*'),
            _sameCharacterButton('/'),
          ],
        ),
        Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _sameCharacterButton('7'),
            _sameCharacterButton('8'),
            _sameCharacterButton('9'),
            _sameCharacterButton('.'),
          ],
        ),
        Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _sameCharacterButton('4'),
            _sameCharacterButton('5'),
            _sameCharacterButton('6'),
            _sameCharacterButton('x'),
          ],
        ),
        Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.center,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            _sameCharacterButton('1'),
            _sameCharacterButton('2'),
            _sameCharacterButton('3'),
            _buttonWrapper(FilledButton(
              onPressed: onSubmit,
              child: const Text('➜')
            )),
          ],
        ),
        const SizedBox(height: 16,),
      ],
    ),
  );
  
}
