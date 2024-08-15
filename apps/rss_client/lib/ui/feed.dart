import 'package:fast_rss_data_fl/fast_rss_data_fl.dart';
import 'package:flutter/material.dart';
import 'package:url_launcher/url_launcher_string.dart';

class Feed extends StatelessWidget {
  const Feed({super.key, required this.items});

  final List<Item> items;

  Widget _buildCardCompact(Item i) => ListTile(
    title: i.title == null ? null : Text(i.title!),
    subtitle: i.description == null ? null : Text(i.description!),
    onTap: i.link == null ? null : () => launchUrlString(i.link!),
  );

  Widget _buildCard(BuildContext context, Item i) => Card(
    child: Padding(
      padding: const EdgeInsets.all(16.0,),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (i.title != null)
            Text(i.title!, style: Theme.of(context).textTheme.titleLarge),
          if (i.title != null && i.description != null)
            const SizedBox(height: 8.0,),
          if (i.description != null)
            Text(i.description!, style: Theme.of(context).textTheme.bodyLarge),
        ],
      ),
    ),
  );

  @override
  Widget build(BuildContext context) => ListView(
    children: [
      // TODO:
      // - timestamp
      // - details (html content)
      // - potential image
      // - mark as read (guid info is already there)
          for (final i in items)
            _buildCard(context, i),
    ],
  );
}
