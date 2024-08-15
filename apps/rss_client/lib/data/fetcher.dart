import 'dart:io';

import 'package:fast_rss_data_fl/fast_rss_data_fl.dart';

class Fetcher {
  const Fetcher(this.host, this.port);

  final String host;
  final int port;

  Future<RssSummary?> fetch() async {
    final connection = await Socket.connect(host, port);
    final data = await connection.first;
    return decode(data: data, decompress: true);
  }
}

