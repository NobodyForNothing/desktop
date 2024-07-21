import 'package:flutter/material.dart';
import 'persistence.dart';

void main() {
  runApp(RewordleApp());
}

class Defaults {
  static final Color background = Colors.black;
  static final Color textColor = Colors.white;
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
  GameState? state;
  String err = '';

  @override
  void initState() {
    super.initState();
    DayLoader.load('2024-04-13').then((s) => setState((){state = s;})); 
  }

  @override
  void dispose() {
    if (state != null) DayLoader.save('2024-04-13', state!);
    super.dispose();
  }

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
      backgroundColor: Defaults.background,
      appBar: AppBar(
        forceMaterialTransparency: true,
        leading: state == null ? CircularProgressIndicator() : null,
      ),
      body: Column(
        children: [
          GuessesList(guesses: [
            for (final e in state?.submitted ?? [])
	      e,
	    state?.current ?? [], // todo cache letters until loaded
	  ]),
          Keyboard(
	    okLetters: state?.okLetters ?? '',
	    wrongPosLetters: state?.wrongPosLetters ?? '',
	    wrongLetters: state?.wrongLetters ?? '',
            onLetter: (l) {
	      if ((state?.current.length ?? 6) < 5) {
	        setState(() => state!.current.add(LetterData(LetterCorrectness.none, l)));
	      }
	    },
            onDone: () {
	      String word = '';
	      for (final e in state?.current ?? []) {
	        word += e.letter;
	      }
	      setState(() {
	        try {
	        String? resp = state?.addWord(word);
		if (state == null) {
		  resp = 'Loading, please wait';
		}
                if (resp != null) {
	          err = resp;
                } else {
		  err = '';
                  state?.current.clear();
		}
		} catch (e, s) {
		  err = word + e.toString() + s.toString();
		}
              });
	      if (state != null) DayLoader.save('2024-04-13', state!);
	    },
            onBack: () {
              if ((state?.current.length ?? 0) > 0) setState(() => state?.current.removeLast());
	    },
          ),
	  SingleChildScrollView(child: Text(err, style: TextStyle(color: Colors.white))),
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
       style: TextStyle(
         fontSize: Defaults.textSize,
         fontWeight: FontWeight.bold,
	 color: Defaults.textColor,
       ),
    ));
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
              child: Center(child: Text("ENTER", style: TextStyle(color: Defaults.textColor))),
            ),
          ),
          for (final l in "ZXCVBNM".split(""))
            _letterBtn(l),
          InkWell(
            onTap: onBack,
            child: Container(
              width: 50.0,
              height: 45.0,
              child: Center(child: Icon(Icons.backspace, color: Defaults.textColor))),
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


