import 'package:shared_preferences/shared_preferences.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:rewordle/valid_words.dart';

/// A persistence and external storage manager.
class DayLoader {
  /// Load a days GameState from memory or the wordle api.
  ///
  /// The method will recursively continue until a response is available.
  ///
  /// [day] is expected to be in the wordle api format likke returned by the [WordleFormat] extension.
  static Future<GameState> load(String day, [SharedPreferences? prefs]) async {
    prefs ??= await SharedPreferences.getInstance();

    final data = prefs.getString(day);
    if (data != null) {
      return GameState.deserialize(data);
    } else {
      //final response = await http.get(Uri.parse('https://www.nytimes.com/svc/wordle/v2/2022-08-14.json'));
      final response = await http.get(Uri.parse(
          'https://corsproxy.io/?https%3A%2F%2Fwww.nytimes.com%2Fsvc%2Fwordle%2Fv2%2F$day.json'));
      if (response.statusCode != 200) return load(day, prefs);
      final data = jsonDecode(response.body) as Map<String, dynamic>;
      final word = data['solution'].toUpperCase();
      return GameState(word);
    }
  }

  /// Store a days game state for later [load]ing.
  static Future<void> save(String day, GameState data,
      [SharedPreferences? prefs]) async {
    prefs ??= await SharedPreferences.getInstance();
    await prefs.setString(day, data.serialize());
  }
}

/// State and logic of a days wordle game.
class GameState {
  GameState(this.correctWord);

  /// Load a [serialize]d game state.
  factory GameState.deserialize(String data) {
    final e = data.split('|');

    final state = GameState(e[0]);
    state.wrongLetters = e[1];
    state.wrongPosLetters = e[2];
    state.okLetters = e[3];

    final words = e[4].split(',');
    words.forEach(state.addWord);
    return state;
  }

  final List<LetterData> current = [];
  final List<List<LetterData>> submitted = [];
  final String correctWord;

  String wrongLetters = '';
  String wrongPosLetters = '';
  String okLetters = '';

  bool finished = false;

  /// Store a wordle atate for later deserialization.
  String serialize() {
    String submissionsString = '';
    for (final wData in submitted) {
      String w = '';
      for (final l in wData) {
        w += l.letter;
      }
      submissionsString += '$w,';
    }
    if (submitted.isNotEmpty) {
      // remove trailing ,
      submissionsString =
          submissionsString.substring(0, submissionsString.length - 1);
    }
    return '$correctWord|$wrongLetters|$wrongPosLetters|$okLetters|$submissionsString';
  }

  /// Add a word and update corresponding instance variables.
  String? addWord(String word, [bool skipWordListValidation = false]) {
    final letters = word.split('');
    if (letters.length != 5) return 'Not 5 letters long';
    if (!(skipWordListValidation || VALID_WORDS.contains(word))) return 'Not in word list';
    final correct = correctWord.split('');

    final checked = <LetterData>[];
    for (int i = 0; i < 5; i++) {
      if (correct[i] == letters[i]) {
        correct[i] = '';
	checked.add(LetterData(LetterCorrectness.ok, letters[i]));
      } else {
        checked.add(LetterData(LetterCorrectness.err, letters[i]));
      }
    }
    for (int i = 0; i < 5; i++) {
      for (int j = 0; j < 5; j++) {
        if (correct[i] == letters[j]) {
	  correct[i] = '';
	  checked[j] = LetterData(LetterCorrectness.warn, letters[j]);
	}
      }
    }

    for (final l in checked) {
      switch(l.state) {
        case LetterCorrectness.ok:
	  okLetters += l.letter;
        case LetterCorrectness.warn:
	  wrongPosLetters += l.letter;
        case LetterCorrectness.err:
	  wrongLetters += l.letter;
	case LetterCorrectness.none:
      }
    }

    submitted.add(checked);

    finished = () {
      for (final l in checked) {
        if (l.state != LetterCorrectness.ok) return false;
      }
      return true;
    }();
    return null;
  }
}

/// A single [letter] and its validation [state].
class LetterData {
  final LetterCorrectness state;
  /// A one character(A-Z) long string.
  final String letter;

  const LetterData(this.state, this.letter);
}

/// Validation state of a single wordle letter.
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

/// Utilities for converting between [DateTime] and wordle api strings.
extension WordleFormat on DateTime {
  /// Output this date as a wordle api compatible date.
  ///
  /// Example: `2024-02-23`
  String wFormat() => "${year.toString()}-${month.toString().padLeft(2, '0')}-${day.toString().padLeft(2, '0')}";
}
