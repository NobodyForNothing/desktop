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

  /// Keyboard with value input, basic operators and submit button.
  Widget _buildPrimaryKeyboard(BuildContext context) => Column(
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
          _buttonWrapper(Tooltip(
            message: 'compute result',
            child: FilledButton(
              onPressed: onSubmit,
              child: const Text('➜')
            ),
          )),
        ],
      ),
    ],
  );

  Widget _buildSecondaryKeyboard(BuildContext context) => Column(
    children: [
      Row(
        children: [
          _characterButton('√', () => onEntered('sqrt(')),
          _characterButton('π', () => onEntered('pi')),
          _characterButton('e', () => onEntered('e')),
        ],
      ),
      Row(
        children: [
          _characterButton('sin', () => onEntered('sin(')),
          _characterButton('cos', () => onEntered('cos(')),
          _characterButton('tan', () => onEntered('tan(')),
        ],
      ),
      Row(
        children: [
          _characterButton('sin⁻¹', () => onEntered('asin(')),
          _characterButton('cos⁻¹', () => onEntered('acos(')),
          _characterButton('tan⁻¹', () => onEntered('atan(')),
        ],
      ),
      Row(
        children: [
          _sameCharacterButton('('),
          _sameCharacterButton(')'),
          _sameCharacterButton('='),
        ],
      ),
    ],
  );

  @override
  Widget build(BuildContext context) {
    final primaryKB = _buildPrimaryKeyboard(context);
    final secondaryKB = _buildSecondaryKeyboard(context);
    return Padding(
      padding: const EdgeInsets.all(12.0),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.center,
        mainAxisAlignment: MainAxisAlignment.center,
        mainAxisSize: MainAxisSize.min,
        children: [
          primaryKB,
          const VerticalDivider(),
          secondaryKB,
        ],
      ),
    );
    // TODO: swiping when not enough space
  }
  
}
