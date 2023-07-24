import 'dart:core';

class Message {
  late String _id;
  late String _senderID;
  late String _chatID;
  late String _content;
  late DateTime _timeSent;

  Message.fromJson(Map<String, dynamic> json) {
    _id = json["id"];
    _senderID = json["sender_id"];
    _chatID = json["chat_id"];
    _content = json["content"];
    _timeSent = DateTime.parse(json["time_sent"]);
  }

  String getID() => _id;
  String getSenderID() => _senderID;
  String getChatId() => _chatID;
  String getContent() => _content;
  DateTime getTimeSent() => _timeSent;

  @override
  String toString() {
    return "ID: $_id\nSender: $_senderID\nChat: $_chatID\nContent: $_content\nTime: $_timeSent\n";
  }
}
