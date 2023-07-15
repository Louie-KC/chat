import 'package:flutter/material.dart';
import 'package:chat_app/service/chat_api_service.dart';

class ApiTestView extends StatefulWidget {
  const ApiTestView({super.key});

  @override
  State<StatefulWidget> createState() => _ApiTestView();
}

class _ApiTestView extends State<ApiTestView> {
  String lastResponse = "";

  ChatApi api = ChatApi();

  void healthCheck() async {
    lastResponse = await api.healthCheck();
    debugPrint(lastResponse);
    setState(() {});
  }

  void createAccount() async {
    lastResponse = await api.createAccount("createtest", "eee");
    debugPrint(lastResponse);
    setState(() {});
  }

  void login() async {
    lastResponse = await api.login("createtest", "eee");
    debugPrint(lastResponse);
    setState(() {});
    debugPrint("API has token: ${api.hasToken()}");
  }

  @override
  Scaffold build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Api Test Page"),
      ),
      body: Padding(
        padding: const EdgeInsets.all(8),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Text("last response: $lastResponse"),
              ElevatedButton(
                onPressed: healthCheck,
                child: const Text("health check"),
              ),
              ElevatedButton(
                onPressed: createAccount,
                child: const Text("create account"),
              ),
              ElevatedButton(
                onPressed: login,
                child: const Text("login"),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
