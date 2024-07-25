import 'package:rewordle/persistence.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('deserializes serialized empty GameStates', () {
    final initial = GameState('ABOUT');
    final loaded = GameState.deserialize(initial.serialize());
    expect(initial.correctWord, equals(loaded.correctWord));
  });
  test('GameState serializtion works', () {
    final init = GameState('TESTS');
    expect(init.addWord('ABOUT'), isNull);
    expect(init.addWord('REPLY'), isNull);

    final l = GameState.deserialize(init.serialize());
    expect(l.correctWord, init.correctWord);
    expect(l.submitted, hasLength(2));
    expect(l.submitted[0][0].letter, "A");
    expect(l.submitted[0][4].letter, "T");
    expect(l.submitted[1][4].letter, "Y");
  });
  test('determines letters at correct positions and not in word', () {
    final s = GameState('AAAAA');
    expect(s.addWord('ABBAA'), isNull);

    expect(s.submitted[0][0].state, LetterCorrectness.ok);
    expect(s.submitted[0][1].state, LetterCorrectness.err);
    expect(s.submitted[0][2].state, LetterCorrectness.err);
    expect(s.submitted[0][3].state, LetterCorrectness.ok);
    expect(s.submitted[0][4].state, LetterCorrectness.ok);
  });
  test('determines letters at wrong positions', () {
    final s = GameState('ABCDE');
    expect(s.addWord('XCXEE'), isNull);

    expect(s.submitted[0][0].state, LetterCorrectness.err);
    expect(s.submitted[0][1].state, LetterCorrectness.warn);
    expect(s.submitted[0][2].state, LetterCorrectness.err);
    expect(s.submitted[0][3].state, LetterCorrectness.warn);
    expect(s.submitted[0][4].state, LetterCorrectness.ok);
  });


}
