import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:chat_app/controllers/chat_controller.dart';
import 'package:chat_app/view/common/header.dart';
import 'package:chat_app/models/chat.dart';
import 'package:chat_app/models/message.dart';

class ChatPage extends StatefulWidget {
  final String chatID;

  const ChatPage({super.key, required this.chatID});

  @override
  State<StatefulWidget> createState() => _ChatPage();
}

class _ChatPage extends State<ChatPage> {
  Widget _messageBox(Message msg, bool fromUser) {
    BoxDecoration decor = BoxDecoration(
      borderRadius: BorderRadius.circular(24),
      color: fromUser ? Colors.blue : Colors.black12,
    );

    return Align(
      alignment: fromUser ? Alignment.centerRight : Alignment.centerLeft,
      child: Container(
        height: 48,
        decoration: decor,
        padding: const EdgeInsets.all(16),
        child: Text(msg.getContent()),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final ChatController controller = Provider.of<ChatController>(context);

    controller.retrieveChatMessages(widget.chatID);
    Future.delayed(const Duration(seconds: 1));

    return Scaffold(
      appBar: Header(
        title: controller.getActiveChat().chatName,
        showBack: true,
      ),
      body: Container(
        padding: const EdgeInsets.symmetric(horizontal: 8),
        width: double.infinity,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          children: controller
              .getActiveChat()
              .messages
              .map((msg) => Padding(
                    padding: const EdgeInsets.all(2.0),
                    child: _messageBox(
                      msg,
                      msg.getSenderID().compareTo("0") == 0, // DEV TEST
                    ),
                  ))
              .toList(),
        ),
      ),
    );
  }
}
