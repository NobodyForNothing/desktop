import 'dart:ffi';

import 'package:fast_rss_data/src/bridge_generated.dart';

typedef ExternalLibrary = DynamicLibrary;

FastRSSData createWrapperImpl(ExternalLibrary dylib) =>
    FastRSSDataImpl(dylib);
