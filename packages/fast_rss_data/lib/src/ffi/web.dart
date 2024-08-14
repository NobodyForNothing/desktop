import 'package:fast_rss_data/src/bridge_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

typedef ExternalLibrary = WasmModule;

FastRSSData createWrapperImpl(ExternalLibrary module) =>
    FastRSSDataImpl.wasm(module);
