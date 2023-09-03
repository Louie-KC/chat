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
  List<Chat> displayChats = List.empty();
  Chat activeChat = Chat.previewOne(Message.fromJson({
    "id": "",
    "sender_id": "",
    "chat_id": "",
    "content": "",
    "time_sent": DateTime.now().toIso8601String(),
  }));
  TextEditingController searchCtrl = TextEditingController(text: "");

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
    displayChats = chats;

    notifyListeners();
    return;
  }

  void updateDisplayedChats(String searchText) {
    displayChats = List.from(
        chats.where((ch) =>
            ch.chatName.contains(searchText) ||
            ch.lastSender.contains(searchText) ||
            ch.lastMessage.getContent().contains(searchText)),
        growable: false);
    notifyListeners();
  }

  List<Chat> getDisplayedChats() => displayChats;

  Future<void> retrieveChatMessages(String chatID) async {
    // DateTime since = displayMessages.isEmpty
    //     ? DateTime(1)
    //     : displayMessages.first.getTimeSent();
    // displayMessages = await api.getChatMessages(chatID, since, _token);

    debugPrint("!!! ChatController::retrieveChatMessages() DEV TEST DATA !!!");
    Message m1 = Message.fromJson({
      "id": "0",
      "sender_id": "0",
      "chat_id": "0",
      "content": "test message 1",
      "time_sent": "${DateTime.now().subtract(const Duration(days: 10))}"
    });
    Message m2 = Message.fromJson({
      "id": "1",
      "sender_id": "0",
      "chat_id": "0",
      "content": "test message 2",
      "time_sent": "${DateTime.now().subtract(const Duration(days: 10))}"
    });
    Message m3 = Message.fromJson({
      "id": "2",
      "sender_id": "1",
      "chat_id": "0",
      "content": "test message 3 - from sender 1",
      "time_sent": "${DateTime.now().subtract(const Duration(days: 10))}"
    });
    Message m4 = Message.fromJson({
      "id": "3",
      "sender_id": "0",
      "chat_id": "0",
      "content": "test message 4",
      "time_sent": "${DateTime.now().subtract(const Duration(days: 10))}"
    });
    Message m5 = Message.fromJson({
      "id": "4",
      "sender_id": "1",
      "chat_id": "0",
      "content": "test message 5 - from sender 1",
      "time_sent": "${DateTime.now().subtract(const Duration(days: 10))}"
    });
    Message m6 = Message.fromJson({
      "id": "5",
      "sender_id": "1",
      "chat_id": "0",
      "content": "test message 6 - from sender 1",
      "time_sent": "${DateTime.now().subtract(const Duration(days: 10))}"
    });
    List<Message> devTestData = <Message>[m1, m2, m3, m4, m5, m6];

    devTestData.sort((a, b) => a.getTimeSent().compareTo(b.getTimeSent()));
    // displayMessages = devTestData;
    activeChat = Chat.full("0", "Dev Test Chat Name", devTestData);

    notifyListeners();
    return;
  }

  Chat getActiveChat() => activeChat;

  // List<Message> getDisplayedMessages() => displayMessages;
}
