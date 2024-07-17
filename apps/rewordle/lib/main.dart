import 'package:flutter/material.dart';

void main() {
  runApp(RewordleApp());
}

class RewordleApp extends StatelessWidget {
  @override
  Widget build(context) => MaterialApp(
    home: Scaffold(
      body: Column(
        children: [
          GuessesList(guesses: []),
          Keyboard(),
        ],
      )
    ),
  );
}

class GuessesList extends StatelessWidget {
  const GuessesList({required this.guesses});

  /// list of list containing all made guesses and the current input.
  final List<List<LetterData>> guesses;

  @override
  Widget build(context) => Padding(
    padding: EdgeInsets.all(12.0),
    child: Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        for (int i = 0; i < 6; i++)
          Guess(letters: guesses.getOrNull(i) ?? []),
      ],
    ),
  );
}

class Guess extends StatelessWidget {
  final List<LetterData> letters;

  const Guess({required this.letters});

  @override
  Widget build(context) => Center(
    child: Row(
    mainAxisSize: MainAxisSize.min,
    children: [
      for (int i = 0; i < 5; i++)
        Letter(letters.getOrNull(i)),
      ],
    ),
  );
}

class Letter extends StatelessWidget {
  final LetterData? l;
  const Letter(this.l);

  @override
  Widget build(context) {
    final w = 55.0;
    final h = 55.0;
    final letter = Center(child: Text(l?.letter ?? ""));
    final box = switch (l?.state) {
      null => Container(
        width: w,
        height: h,
        color: Colors.grey,
       
      ),
      LetterCorrectness.ok => Container(
        width: w, height: h,
        color: Colors.lime,
        child: letter,
      ),
      LetterCorrectness.warn => Container(
        width: w, height: h,
        color: Colors.orange,
        child: letter,
      ),
      LetterCorrectness.err =>Container(
        width: w, height: h,
        color: Colors.red,
        child: letter,
      ),
    };

    return Padding(
      padding: EdgeInsets.all(4.0),
      child: box,
    );
  }
}

class LetterData {
  final LetterCorrectness state;
  final String letter;

  const LetterData(this.state, this.letter);


}

enum LetterCorrectness {
  /// At correct position.
  ok,
  /// At wong position.
  warn,
  /// Not in word.
  err,
}

class Keyboard extends StatelessWidget {
  Widget build(c) => SizedBox();// todo
}

extension ListExtension<T> on List<T> {
  T? getOrNull(int index) {
    return (index >= 0 && index < length) ? this[index] : null;
  }
}
