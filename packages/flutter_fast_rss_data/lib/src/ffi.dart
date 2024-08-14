import 'package:library_name/library_name.dart';
import 'ffi/stub.dart'
if (dart.library.io) 'ffi/io.dart'
if (dart.library.js_interop) 'ffi/web.dart';

LibraryName createLib() =>
    createWrapper(createLibraryImpl());