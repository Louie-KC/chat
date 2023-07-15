import 'package:flutter/material.dart';
import 'package:chat_app/api_test/api_test_view.dart';

void main() {
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: ApiTestView(),
    );
  }
}
