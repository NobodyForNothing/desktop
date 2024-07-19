import 'package:flutter/material.dart';

void main() {
  runApp(RewordleApp());
}

class Defaults {
  static final Color background = Colors.black;
  static final Color textColor = Colors.white;
  static final Color buttonBg = Colors.grey;
  static final double textSize = 24.0;
  static final Color correctPos = Color(0xFF669F5E);
  static final Color wrongPos = Color(0xFFC8B557);
  static final Color notInWord = Color(0xFF919293);
}

class RewordleApp extends StatefulWidget {
  @override
  State<RewordleApp> createState() => _RewordleAppState();
}

class _RewordleAppState extends State<RewordleApp> {
  final List<LetterData> current = [];
  final List<List<LetterData>> submitted = [];
  final word = "ABOUT";

  String wrongLetters = "";
  String wrongPosLetters = "";
  String okLetters = "";

  @override
  Widget build(context) => MaterialApp(
    theme: ThemeData(
      backgroundColor: Defaults.background,
      canvasColor: Defaults.background,
    ),	
    home: DefaultTextStyle(
     style: TextStyle(
       fontSize: Defaults.textSize,
       fontWeight: FontWeight.bold,
     ),
     child: Scaffold(
      body: Column(
        children: [
          GuessesList(guesses: [
            for (final e in submitted)
	      e,
	    current,
	  ]),
          Keyboard(
	    okLetters: okLetters,
	    wrongPosLetters: wrongPosLetters,
	    wrongLetters: wrongLetters,
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
		  setState(()=>okLetters += l);
		  checked.add(LetterData(LetterCorrectness.ok, l));
		} else if (word.contains(l)) {
		  setState(()=>wrongPosLetters += l);
		  checked.add(LetterData(LetterCorrectness.warn, l));
		} else {
		  setState(()=>wrongLetters += l);
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
    final w = 57.0;
    final h = 57.0;
    final letter = Center(child: Text(l?.letter ?? "",
      style: TextStyle(fontSize: Defaults.textSize)));
    final box = switch (l?.state) {
      LetterCorrectness.ok => Container(
        width: w, height: h,
        color: Defaults.correctPos,
        child: letter,
      ),
      LetterCorrectness.warn => Container(
        width: w, height: h,
        color: Defaults.wrongPos,
        child: letter,
      ),
      LetterCorrectness.err =>Container(
        width: w, height: h,
        color: Defaults.notInWord,
        child: letter,
      ),
      null || LetterCorrectness.none => Container(
        width: w,
	height : h,
	decoration: BoxDecoration(
	  border: Border.all(width: 2.0, color: Defaults.notInWord),
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
  const Keyboard({
    required this.onLetter,
    required this.onDone,
    required this.onBack,
    required this.okLetters,
    required this.wrongPosLetters,
    required this.wrongLetters,
  });

  final void Function(String) onLetter;
  final void Function() onDone;
  final void Function() onBack;

  final String okLetters;
  final String wrongPosLetters;
  final String wrongLetters;

  // todo: coloring
  Widget _letterBtn(String letter) => InkWell(
    onTap: () => onLetter(letter),
    child: Container(
      width: 36.0,
      height: 54.8,
      decoration: BoxDecoration(
        color: (){
         if (okLetters.contains(letter)) {
	   return Defaults.correctPos;
	 } else if (wrongPosLetters.contains(letter)) {
	   return Defaults.wrongPos;
	 } else if (wrongLetters.contains(letter)) {
	   return Color(0xff2c3033);
	 }
	 return Color(0xff5e6669);
	}(),
	borderRadius: BorderRadius.all(Radius.circular(5.0)),
      ),
      margin: EdgeInsets.all(1.5),
      child: Center(
        child: Text(letter, style: TextStyle(
	  color: Defaults.textColor,
          fontSize: Defaults.textSize,
	  fontWeight: FontWeight.bold,
	  ))
	),
     )
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


