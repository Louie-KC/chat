import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:chat_app/view/pages/login_page.dart';
import 'package:chat_app/view/pages/home_page.dart';
import 'package:chat_app/view/pages/chat_page.dart';
import 'package:chat_app/controllers/chat_controller.dart';
// import 'package:chat_app/api_test/api_test_view.dart';

void main() {
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (_) => ChatController(),
      child: MaterialApp(
        home: ChatPage(chatID: "0"),
      ),
    );
  }
}
