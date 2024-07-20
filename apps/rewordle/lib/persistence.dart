import 'package:shared_preferences/shared_preferences.dart';

class DayLoader {
  static Future<GameState> load(String day, [SharedPreferences? prefs]) async {
    prefs ??= await SharedPreferences.getInstance();

    final data = prefs.getString(day);
    if (data != null) {
      return GameState.deserialize(data);
    } else {
      final word = "TODOS";
      return GameState(word);
    }
  }
}

class GameState {
  GameState(this.correctWord);

  factory GameState.deserialize(String data) {
    final e = data.split("|");
    
    final state = GameState(e[0]);
    state.wrongLetters = e[1];
    state.wrongPosLetters = e[2];
    state.okLetters = e[3];

    final words = e[4].split(',');
    final correct = e[0].split('');
    for (final w in words) {
      final wSub = <LetterData>[];
      final letters = w.split('');
      for (int i = 0; i < 5; i++) {
        if (correct[i] == letters[i]) {
          wSub.add(LetterData(LetterCorrectness.ok, letters[i]));
	} else if (e[0].contains(letters[i])) {
          wSub.add(LetterData(LetterCorrectness.warn, letters[i]));
	} else {
	  wSub.add(LetterData(LetterCorrectness.err, letters[i]));
	}
      }
      state.submitted.add(wSub);
    }
    // todo: stop duplicating this algorithm
    return state;
  }

  final List<LetterData> current = [];
  final List<List<LetterData>> submitted = [];
  final String correctWord;

  String wrongLetters = "";
  String wrongPosLetters = "";
  String okLetters = "";
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



