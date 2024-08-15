import 'package:flutter/material.dart';
import 'package:fast_rss_data_fl/fast_rss_data_fl.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    final data = encode(data: RssSummary(data: [Channel(title: "test", items: [])]), compress: true)!;
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
              'Action: Decode sample data\nResult: `${decode(data: data, decompress: true)!.data[0].title}`'),
        ),
      ),
    );
  }
}
