import 'package:fast_rss_data_fl/fast_rss_data_fl.dart';
import 'package:flutter/material.dart';
import 'package:rss_client/ui/channel.dart';

import 'data/fetcher.dart';

void main() async {
  await RustLib.init();
  final data = await const Fetcher('10.0.2.2', 5678).fetch();
  runApp(MaterialApp(home: Scaffold(body: Feed(channel: data!.data[0]),),));
}
