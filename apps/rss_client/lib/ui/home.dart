import 'package:fast_rss_data_fl/fast_rss_data_fl.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:rss_client/ui/feed.dart';

class Home extends StatelessWidget {
  const Home({super.key, this.data});

  final RssSummary? data;

 @override
  Widget build(BuildContext context) => DefaultTabController(
    initialIndex: 0,
    length: data?.data.length ?? 1,
    child: Scaffold(
      appBar: AppBar(
        forceMaterialTransparency: true,
        title: (data?.data.length ?? 0) > 0
          ? TabBar(
            isScrollable: true,
            dragStartBehavior: DragStartBehavior.start,
            tabAlignment: TabAlignment.start,
            padding: EdgeInsets.zero,
            tabs: [
              for (final f in data?.data ?? <Channel>[])
                SizedBox(
                  height: 40,
                  child: Center(
                    child: Text(f.title ?? f.link ?? 'untitled')
                  )
                ),
            ],
          )
          : null,
      ),
      body: TabBarView(
        children: [
          for (final f in data?.data ?? <Channel>[])
            Feed(items: f.items)
        ],
      ),
    ),
  );
}

