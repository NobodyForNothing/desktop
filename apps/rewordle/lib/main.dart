import 'package:flutter/material.dart';

void main() {
  runApp(RewordleApp());
}

class RewordleApp extends StatefulWidget {
  @override
  State<RewordleApp> createState() => _RewordleAppState();
}

class _RewordleAppState extends State<RewordleApp> {
  final List<LetterData> current = [];
  final List<List<LetterData>> submitted = [];
  final word = "ABOUT";

  @override
  Widget build(context) => MaterialApp(
    home: Scaffold(
      body: Column(
        children: [
          GuessesList(guesses: [
            for (final e in submitted)
	      e,
	    current,
	  ]),
          Keyboard(
            onLetter: (l) {
	      if (current.length <= 5) {
	        setState(() => current.add(LetterData(LetterCorrectness.none, l)));
	      }
	    },
            onDone: () {
              if (current.length != 5) {
	        ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: const Text('Wrong length'),
	   	  ),
	        );
		// TODO: word list check
              }
	
    	      final checked = <LetterData>[];
	      for (int i = 0; i < 5; i++) {
                final l = current[i].letter;
                if (word[i] == l) {
		  checked.add(LetterData(LetterCorrectness.ok, l));
		} else if (word.contains(l)) {
		  checked.add(LetterData(LetterCorrectness.warn, l));
		} else {
		  checked.add(LetterData(LetterCorrectness.err, l));
		}
              }

              setState(() {
                submitted.add(checked);
		current.clear();
	      });
	    },
            onBack: () {
              if (current.length > 0) setState(() => current.removeLast());
	    },
          ),
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
        color: Colors.grey,
        child: letter,
      ),
      null || LetterCorrectness.none => Container(
        width: w - 4.0,
	height : h - 4.0,
	decoration: BoxDecoration(
	  border: Border.all(width: 2.0, color: Colors.grey),
        ),
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
  /// Unknown state, not yet submitted.
  none,
}

class Keyboard extends StatelessWidget {
  const Keyboard({required this.onLetter, required this.onDone, required this.onBack});

  final void Function(String) onLetter;
  final void Function() onDone;
  final void Function() onBack;

  // todo: coloring
  Widget _letterBtn(String letter) => InkWell(
    onTap: () => onLetter(letter),
    child: Container(
      width: 30.0,
      height: 45.0,
      child: Center(child: Text(letter)),
    ),
  );

  @override
  Widget build(c) => Column(
    mainAxisAlignment: MainAxisAlignment.center,
    children: [
      Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          for (final l in "QWERTYUIOP".split(""))
            _letterBtn(l),
        ],
      ),
      Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          for (final l in "ASDFGHJKL".split(""))
            _letterBtn(l),
        ],
      ),
      Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          InkWell(
            onTap: onDone,
            child: Container(
              width: 60.0,
              height: 45.0,
              child: Center(child: Text("ENTER")),
            ),
          ),
          for (final l in "ZXCVBNM".split(""))
            _letterBtn(l),
          InkWell(
            onTap: onBack,
            child: Container(
              width: 50.0,
              height: 45.0,
              child: Center(child: Icon(Icons.backspace))),
            ),
        

        ],
      ),
    ],
  );// todo
}

extension ListExtension<T> on List<T> {
  T? getOrNull(int index) {
    return (index >= 0 && index < length) ? this[index] : null;
  }
}


