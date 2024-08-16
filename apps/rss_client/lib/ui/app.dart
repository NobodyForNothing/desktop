import 'dart:io';

import 'package:fast_rss_data_fl/fast_rss_data_fl.dart';
import 'package:flutter/material.dart';
import 'package:rss_client/data/fetcher.dart';

import 'package:rss_client/ui/home.dart';

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) => MaterialApp(
    theme: ThemeData.dark(useMaterial3: true),
    home: FutureBuilder(
      future: Fetcher(Platform.isAndroid ? '10.0.2.2' : '127.0.0.1', 5678).fetch(),
      builder: (BuildContext context, AsyncSnapshot<RssSummary?> snapshot) {
        if (snapshot.hasData) {
          return Home(data: snapshot.data);
        }
        return const Text('loading...');
      },
    ),
  );
}

