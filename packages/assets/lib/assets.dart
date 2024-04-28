library assets;

import 'package:flutter/widgets.dart';
import 'package:flutter_svg_icons/flutter_svg_icons.dart';

class Assets {
  static Widget get genericFile => new SvgIcon(
      icon: SvgIconData('packages/assets/icons/generic_file.svg')
  );
}
