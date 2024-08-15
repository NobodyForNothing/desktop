import 'package:fast_rss_data_fl/fast_rss_data_fl.dart';
import 'package:flutter/material.dart';

class Feed extends StatelessWidget {
  const Feed({super.key, required this.channel});

  final Channel channel;

  @override
  Widget build(BuildContext context) => Column(
    children: [
      Text('title: ${channel.title}'),
      Text('description: ${channel.description}'),
      for (final e in channel.items)
        Text(e.title ?? ''),
    ],
  );
}
