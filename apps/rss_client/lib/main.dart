import 'package:fast_rss_data_fl/fast_rss_data_fl.dart';
import 'package:flutter/material.dart';
import 'package:rss_client/ui/app.dart';
import 'package:rss_client/ui/feed.dart';

import 'data/fetcher.dart';

void main() async {
  await RustLib.init();
  runApp(const App());
}
