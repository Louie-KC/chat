import 'dart:core';

import 'package:chat_app/models/message.dart';

class Chat {
  late String _chatID;
  late String _name;
  late String _lastSenderID;
  late List<Message> _messages;

  Chat.previewOne(Message mostRecent) {
    _chatID = mostRecent.getID();
    _name = "UNDEFINED";
    _lastSenderID = mostRecent.getSenderID();
    _messages = List.filled(1, mostRecent);
  }

  Chat.full(this._chatID, this._name, this._messages) {
    if (_messages.isEmpty) {
      _lastSenderID = "UNDEFINED";
    } else {
      _lastSenderID = _messages.first.getSenderID();
    }
  }

  String get chatID => _chatID;
  String get chatName => _name;
  String get lastSender => _lastSenderID;
  Message get lastMessage => _messages.first;
  List<Message> get messages => _messages;
}
