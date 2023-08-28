import 'package:chat_app/models/message.dart';
import 'package:flutter/material.dart';

import '../models/chat.dart';
import '../models/message.dart';
import '../service/chat_api_service.dart';

class ChatController extends ChangeNotifier {
  final api = ChatApi();
  String username = "";
  String _token = "";
  bool accountProcess = false;
  List<Chat> chats = List.empty();

  bool loggedIn() => _token.isNotEmpty;

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

  // Future<List<Chat>> getChats() async {
  Future<void> getChats() async {
    // DateTime since =
    //     chats.isEmpty ? DateTime(1) : chats.first.lastMessage.getTimeSent();
    // chats = await api.getChatOverview(since, _token);
    // chats.sort((a, b) =>
    //     a.lastMessage.getTimeSent().compareTo(b.lastMessage.getTimeSent()));
    // return chats;

    // dev test
    debugPrint("!!! ChatController::getChats() DEV TEST DATA IN USE !!!");
    Message m1 = Message.fromJson({
      "id": "0",
      "sender_id": "0",
      "chat_id": "0",
      "content": "test message 1",
      "time_sent": "${DateTime.now().subtract(const Duration(days: 10))}"
    });
    Message m2 = Message.fromJson({
      "id": "2",
      "sender_id": "0",
      "chat_id": "1",
      "content": "test message 2",
      "time_sent": "${DateTime.now().subtract(const Duration(days: 8))}"
    });
    Message m3 = Message.fromJson({
      "id": "0",
      "sender_id": "1",
      "chat_id": "5",
      "content": "test message 3, most recent",
      "time_sent": "${DateTime.now().subtract(const Duration(days: 1))}"
    });
    List<Chat> devTestData = <Chat>[
      Chat.previewOne(m1),
      Chat.previewOne(m2),
      Chat.previewOne(m3),
    ];
    devTestData.sort((a, b) =>
        b.lastMessage.getTimeSent().compareTo(a.lastMessage.getTimeSent()));
    chats = devTestData;

    notifyListeners();
    return;
  }
}
