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

  static Future<void> save(String day, GameState data, [SharedPreferences? prefs]) async {
    prefs ??= await SharedPreferences.getInstance();
    await prefs.setString(day, data.serialize());
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
    words.forEach(state.addWord);
    return state;
  }

  final List<LetterData> current = [];
  final List<List<LetterData>> submitted = [];
  final String correctWord;

  String wrongLetters = "";
  String wrongPosLetters = "";
  String okLetters = "";

  String serialize() {
    String submissionsString = "";
    for (final wData in submitted) {
      String w = "";
      for (final l in wData) {
        w += l.letter;
      }
      submissionsString += '$w,';
    }
    if (submitted.isNotEmpty) { // remove trailing ,
      submissionsString = submissionsString.substring(0,submissionsString.length - 1);
    }
    return '$correctWord|$wrongLetters|$wrongPosLetters|$okLetters|$submissionsString';
  }

  String? addWord(String word) {
    final letters = word.split('');
    if (letters.length != 5) return 'Not 5 letters long';
    // TODO: word list check

    final checked = <LetterData>[];
    for (int i = 0; i < 5; i++) {
      final l = letters[i];
      if (correctWord[i] == l) {
        okLetters += l;
        checked.add(LetterData(LetterCorrectness.ok, l));
      } else if (correctWord.contains(l)) {
        wrongPosLetters += l;
	checked.add(LetterData(LetterCorrectness.warn, l));
      } else {
        wrongLetters += l;
	checked.add(LetterData(LetterCorrectness.err, l));
      }
    }

    submitted.add(checked);

    return null;
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



