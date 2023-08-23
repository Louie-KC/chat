import 'package:flutter/material.dart';

import '../service/chat_api_service.dart';

class ChatController extends ChangeNotifier {
  final api = ChatApi();
  String username = "";
  String _token = "";
  bool accountProcess = false;

  Future<void> login(String uname, String pword) async {
    if (accountProcess) {
      return;
    }
    accountProcess = true;
    _token = await api
        .login(uname, pword)
        .timeout(const Duration(seconds: 5), onTimeout: () => "");
    if (_token.isNotEmpty) {
      username = uname;
    }
    accountProcess = false;
    notifyListeners();
  }

  Future<void> register(String uname, String pword) async {
    if (accountProcess) {
      return;
    }
    accountProcess = true;
    await api.createAccount(uname, pword).timeout(const Duration(seconds: 5));
    accountProcess = false;

    return;
  }

  bool loggedIn() => _token.isNotEmpty;
}
