import 'package:fast_rss_data/src/bridge_generated.dart';

/// Represents the external library for fast_rss_data
///
/// Will be a DynamicLibrary for dart:io or WasmModule for dart:html
typedef ExternalLibrary = Object;

FastRSSData createWrapperImpl(ExternalLibrary lib) =>
    throw UnimplementedError();
