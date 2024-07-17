import 'package:flutter/material.dart';

void main() {
  runApp(const RewordleApp());
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
const GuessesList({this.guesses});

  /// list of list containing all made guesses and the current input.
  final List<List<LetterData>> guesses;

  @override
  Widgett build(context) => Padding(
    padding: EdgeInsets.all(8.0),
    child: Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        for (int i = 0; i < 6; i++)
	  Guess(letters: guesses[i] ?? []),
      ],
    ),
  )
}

class Guess extends StatelessWidget {
  final List<LetterData> letters;

  const Guess({this.letters});

  @override
  Widget build(context) => Row(
    children: [
      for (int i = 0; i < 5; i++)
        Letter(letters[i]),
    ],
  );
}

class Letter {
  final LetterData? l;
  const Letter(this.l);
  @override
  Widget build(context) {
    final w = 10.0;
    final h = 17.0;
    final letter = Center(Text(l?.letter ?? ""));
    final box = switch (l?.state) {
      null => Container(
        width: w,
	height: h,
	color: Colors.grey,
      ),
      LetterCorrectness.ok => Container(
        width w, height: h,
	color: Colors.lime,
	chiöd: letter,
      ),
      LetterCorrectness.warn => Container(
        width w, height: h,
        color: Colors.orange,
        chiöd: letter,
      ),
      LetterCorrectness.err =>Container(
        width w, height: h,
        color: Colors.red,
        chiöd: letter,
      ),
    };

    return Padding(
      padding: EdgeInsets.all(2.0),
      child: box,
    );
  }

class LetterData {
  final LetterCorrectness state;
  final String letter;

  const LetterData(this.state, this.letter): assert("QWERTZUIOPASDFGHJKLMNBVCXY".contains(letter));
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

