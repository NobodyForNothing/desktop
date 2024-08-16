import 'package:flutter/material.dart';
import 'package:fast_rss_data_fl/fast_rss_data_fl.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(MaterialApp(
      home: Scaffold(
      appBar: AppBar(title: const Text('flutter_rust_bridge example')),
        body: const Center(
          child: MyApp(),
        )
      )
    )
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return FutureBuilder(
      future: Future(() async {
        final data = (await encode(data: RssSummary(data: [Channel(title: "test", items: [])]), compress: true))!;
        return (await decode(data: data, decompress: true))!;
      }),
      builder: (context, snapshot) {
        if (snapshot.data == null) return Text('loading...');
        return Text('Action: Decode sample data\nResult: `${snapshot.data!.data[0].title}`');
      }
    );

  }
}
