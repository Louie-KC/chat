import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:chat_app/models/chat.dart';
import 'package:chat_app/controllers/chat_controller.dart';

class HomePage extends StatelessWidget {
  // late final TextEditingController _searchFieldController;

  HomePage({super.key}) {
    // _searchFieldController = TextEditingController();
  }

  Widget _chatButton(ChatController controller, Chat chat) {
    String lastMsg = chat.lastMessage.getContent();
    if (lastMsg.length > 25) {
      lastMsg = "${lastMsg.substring(0, 22)}...";
    }
    DateTime timestamp = chat.lastMessage.getTimeSent();
    String sender = chat.lastSender;
    String chatName = chat.chatName;

    return InkWell(
      onTap: () {
        debugPrint("Chat Button Pushed"); // TODO
      },
      child: Container(
        height: 96,
        decoration: const BoxDecoration(
          color: Colors.black26,
          shape: BoxShape.rectangle,
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.start,
          children: [
            Container(
              width: 64 + 16,
              margin: const EdgeInsets.symmetric(horizontal: 8),
              decoration: const BoxDecoration(
                color: Colors.white,
                shape: BoxShape.circle,
              ),
            ),
            Column(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  chatName,
                  style: const TextStyle(fontWeight: FontWeight.bold),
                ),
                Text(timestamp.toString()),
                Text(sender),
                Text(lastMsg),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _searchBar(ChatController chatController) => TextField(
        // controller: chatController.searchCtrl,
        decoration: InputDecoration(
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(24),
          ),
          focusedBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(24),
          ),
          hintText: "Search chats",
        ),
        onChanged: (searchText) =>
            chatController.updateDisplayedChats(searchText),
      );

  @override
  Widget build(BuildContext context) {
    final ChatController controller = Provider.of<ChatController>(context);

    return Scaffold(
      appBar: AppBar(
        elevation: 0,
        backgroundColor: Colors.white,
        title: const Text(
          "Home",
          style: TextStyle(color: Colors.black),
        ),
        centerTitle: true,
        automaticallyImplyLeading: false,
      ),
      body: SingleChildScrollView(
        child: Column(
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
            Padding(
              padding: const EdgeInsets.all(4.0),
              child: _searchBar(controller),
            ),
            ElevatedButton(
              onPressed: () => controller.getChats(),
              child: const Text("get chats"),
            ),
            Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: controller
                  .getDisplayedChats()
                  .map((chat) => Padding(
                        padding: const EdgeInsets.all(4.0),
                        child: _chatButton(controller, chat),
                      ))
                  .toList(),
            )
          ],
        ),
      ),
    );
  }
}
