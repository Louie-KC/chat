import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:chat_app/models/chat.dart';
import 'package:chat_app/controllers/chat_controller.dart';

class HomePage extends StatelessWidget {
  // late final TextEditingController _searchFieldController;

  HomePage({super.key}) {
    // _searchFieldController = TextEditingController();
  }

  Widget chatButton(Chat chat) {
    String lastMsg = chat.lastMessage.getContent();
    if (lastMsg.length > 25) {
      lastMsg = "${lastMsg.substring(0, 22)}...";
    }
    DateTime timestamp = chat.lastMessage.getTimeSent();
    String sender = chat.lastSender;
    return Padding(
      padding: const EdgeInsets.all(4),
      child: ElevatedButton(
        onPressed: () => {},
        child: Column(
          children: [
            Text(sender),
            Text(lastMsg),
            Text(timestamp.toString()),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final ChatController controller = Provider.of<ChatController>(context);

    return Scaffold(
      appBar: AppBar(
        elevation: 0,
        backgroundColor: Colors.white,
        title: const Text("Home"),
        automaticallyImplyLeading: false,
        // leading: ,
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Container(
            height: 150,
            decoration: const BoxDecoration(
              color: Colors.grey,
              shape: BoxShape.rectangle,
            ),
          ),
          ElevatedButton(
            onPressed: () => controller.getChats(),
            child: const Text("get chats"),
          ),
          Column(
            children: controller.chats.map((chat) => chatButton(chat)).toList(),
            // .map((chat) => Text(chat.lastMessage.getContent()))
            // .toList(),
          )
        ],
      ),
    );
  }
}
