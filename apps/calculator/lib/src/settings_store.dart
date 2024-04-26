import 'dart:async';

/// Storage of all runtime app settings.
class SettingsStore {
  static final _streamController = StreamController.broadcast();

  /// Stream that receives updates everytime settings change.
  static Stream get stream => _streamController.stream;

  static bool _allowNative = false;
  static bool _hideMath = false;

  /// Whether the native (on-screen) Keyboard should be shown.
  static bool get allowNative => _allowNative;

  static set allowNative(bool value) {
    _streamController.sink.add(null);
    _allowNative = value;
  }

  /// Whether to hide the apps math input.
  static bool get hideMath => _hideMath;

  static set hideMath(bool value) {
    _streamController.sink.add(null);
    _hideMath = value;
  }
}