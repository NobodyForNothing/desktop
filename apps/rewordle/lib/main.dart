import 'package:flutter/material.dart';
import 'package:rewordle/persistence.dart';

void main() => runApp(RewordleApp());

/// Global configuration options.
class Defaults {
  /// Background color of the app.
  static final Color background = Colors.black;
  /// Color of text and icons
  static final Color textColor = Colors.white;
  /// Size of letters on keyboard and word input.
  static final double textSize = 24.0;
  /// Background color of correct letters.
  static final Color correctPos = Color(0xFF669F5E);
  /// Background color of letters at the wrong position.
  static final Color wrongPos = Color(0xFFC8B557);
  /// Background color of letters that are not in the correct word.
  static final Color notInWord = Color(0xFF919293);
}

/// Base of the app, defines structure and logic.
class RewordleApp extends StatefulWidget {
  /// Create the rewordle app.
  const RewordleApp({super.key});

  @override
  State<RewordleApp> createState() => _RewordleAppState();
}

class _RewordleAppState extends State<RewordleApp> {
  GameState? _state;
  String err = '';
  late DateTime today;

  @override
  void initState() {
    super.initState();
    today = DateTime.now();
    final String dateSlug = today.wFormat();
    DayLoader.load(dateSlug).then((s) => setState(() {
      _state = s;
    }));
  }

  @override
  void dispose() {
    if (_state != null) DayLoader.save(today.wFormat(), _state!);
    super.dispose();
  }

  Widget _buildLoadingIndicator() => Padding(
    padding: EdgeInsets.all(3.0),
    child: Stack(
      children: [
        Center(
          child: CircularProgressIndicator(
            valueColor: AlwaysStoppedAnimation<Color>(
              Defaults.textColor,
	    ),
            strokeWidth: 1,
          )
	),
        Center(
          child: Icon(Icons.cloud, color: Defaults.textColor),
	),
      ],
    ),
  );

  @override
  Widget build(BuildContext context) => MaterialApp(
    theme: ThemeData(
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
          leading: _state != null
            ? null
            : _buildLoadingIndicator(),
        ),
        body: Column(
          children: [
            GuessesList(
	      guesses: [
                for (final e in _state?.submitted ?? []) e,
                  _state?.current ?? [], // todo cache letters until loaded
              ],
	    ),
            if (!(_state?.finished ?? false))
              Keyboard(
                okLetters: _state?.okLetters ?? '',
                wrongPosLetters: _state?.wrongPosLetters ?? '',
                wrongLetters: _state?.wrongLetters ?? '',
                onLetter: (l) {
                  if ((_state?.current.length ?? 6) < 5) {
                    setState(() => _state!.current.add(
		      LetterData(LetterCorrectness.none, l),
		      ));
                  }
                },
                onDone: () {
                  String word = '';
                  for (final e in _state?.current ?? []) {
                    word += e.letter;
                  }
                  setState(() {
                    String? resp = _state?.addWord(word);
                    if (_state == null) {
                      resp = 'Loading, please wait';
                    }
                    if (resp != null) {
                      err = resp;
                    } else {
                      err = '';
                      _state?.current.clear();
                    }
                  });

                  if (_state != null) {
                    DayLoader.save(today.wFormat(), _state!);
                  }
                },
                onBack: () {
                  if ((_state?.current.length ?? 0) > 0)
                    setState(() => _state?.current.removeLast());
                },
              ),
              SingleChildScrollView(
                child: Text(err, style: TextStyle(color: Colors.white))),
            ],
          ),
	),
      ),
    );
}

/// List of past submissions current input and remaing attempts.
///
/// Shows past [guesses] and fills up the list of 6 [Guess]es with empty ones.
class GuessesList extends StatelessWidget {
  /// Create a word submission list.
  const GuessesList({required this.guesses});

  /// list of list containing all made guesses and the current input.
  final List<List<LetterData>> guesses;

  @override
  Widget build(BuildContext context) => Padding(
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

/// A wordle entry consisting of exactly 5 [Letter]s.
class Guess extends StatelessWidget {
  const Guess({required this.letters});

  /// Up to five letters making up the word.
  final List<LetterData> letters;

  @override
  Widget build(context) => Center(
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            for (int i = 0; i < 5; i++) Letter(letters.getOrNull(i)),
          ],
        ),
      );
}

/// Single letter [l] on a background indicating correctness.
///
/// Correct warn and err letters are on their respective color configured in settings, unvalidated and null letters are on a transparent background with a border in the color of wrong letters.
class Letter extends StatelessWidget {
  const Letter(this.l);

  final LetterData? l;

  @override
  Widget build(context) {
    final w = 57.0;
    final letter = Center(
        child: Text(
      l?.letter ?? '',
      style: TextStyle(
        fontSize: Defaults.textSize,
        fontWeight: FontWeight.bold,
        color: Defaults.textColor,
      ),
    ));
    final box = switch (l?.state) {
      LetterCorrectness.ok => Container(
          width: w,
          height: w,
          color: Defaults.correctPos,
          child: letter,
        ),
      LetterCorrectness.warn => Container(
          width: w,
          height: w,
          color: Defaults.wrongPos,
          child: letter,
        ),
      LetterCorrectness.err => Container(
          width: w,
          height: w,
          color: Defaults.notInWord,
          child: letter,
        ),
      null || LetterCorrectness.none => Container(
          width: w,
          height: w,
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

/// Shows the wordle keyboard and propagates events.
class Keyboard extends StatelessWidget {
  const Keyboard({
    super.key,
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
      width: 35.0,
      height: 53.5,
      decoration: BoxDecoration(
        color: () {
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
      child: Text(letter,
        style: TextStyle(
          color: Defaults.textColor,
          fontSize: Defaults.textSize,
          fontWeight: FontWeight.bold,
        )
      )
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
              for (final l in 'QWERTYUIOP'.split('')) _letterBtn(l),
            ],
          ),
          Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              for (final l in 'ASDFGHJKL'.split('')) _letterBtn(l),
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
                  child: Center(
                      child: Text('ENTER',
                          style: TextStyle(color: Defaults.textColor))),
                ),
              ),
              for (final l in 'ZXCVBNM'.split('')) _letterBtn(l),
              InkWell(
                onTap: onBack,
                child: Container(
                    width: 50.0,
                    height: 45.0,
                    child: Center(
                        child:
                            Icon(Icons.backspace, color: Defaults.textColor))),
              ),
            ],
          ),
        ],
      ); // todo
}

/// Utility method for generic lists.
extension ListExtension<T> on List<T> {
  /// Return null if [index] is out of range else return the content at that index.
  T? getOrNull(int index) => (index >= 0 && index < length)
  ? this[index]
  : null;
}
