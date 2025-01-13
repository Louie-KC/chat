# API endpoints

## Available endpoints

Admin

* [`GET  /health`](#get-health)

Manage account (`/account`)

* [`POST /account/register`](#post-accountregister)
* [`POST /account/login`](#post-accountlogin)
* [`POST /account/change-password`](#post-accountchange-password)
* [`POST /account/clear-tokens`](#post-accountclear-tokens)

Manage chat room (`/chat`)

* [`GET  /chat/rooms`](#get-rooms)
* [`POST /chat/create-room`](#post-create-room)
* [`PUT  /chat/{room_id}/change-name`](#put-room_idchange-name)
* [`GET  /chat/{room_id}/members`](#get-room_idmembers)
* [`POST /chat/{room_id}/manage-user`](#post-room_idmanage-user)

Chat interaction (`/chat`)

* [`GET  /chat/{room_id}/{offset}/{limit}`](#get-chatroom_idoffsetlimit)
* [`POST /chat`](#post-chat)

### GET /health

Poll the server to see if it is running.

* Authentication: None
* Expected JSON payload: None
* Possible responses:
    * HTTP 200 OK: The server is running

### POST /account/register
Create a new user account.

* Authentication: None
* Expected JSON payload:
```json
{
    "username": <username>,
    "password": <password>
}
```
* Possible responses:
    * HTTP 200 OK: Success.
    * HTTP 400 Bad Request: Disallowed characters in json payload, or incorrect length. See reason.
    * HTTP 500 Internal Server Error: An error has occurred.

### POST /account/login
Login to an existing user account, generating and returning an authentication token on success. 

Note: The returned token is used in all requests where Bearer authentication is required.

* Authentication: None
* Expected JSON payload:
```json
{
    "username": <username>,
    "password": <password>
}
```
* Possible responses:
    * HTTP 200 OK: Success
    ```json
    {
        "token": <auth token>
    }
    ```
    * HTTP 400 Bad Request: Bad login details, or disallowed characters in json payload, or incorrect length. See reason.
    * HTTP 500 Internal Server Error: An error has occurred.

### POST /account/change-password
Change the password of an existing user account.

* Authentication: Bearer
* Expected JSON payload:
```json
{
    "old_password": <old password>,
    "new_password": <new password>
}
```
* Possible responses:
    * HTTP 200 OK: Success
    * HTTP 400 Bad Request:
        * At least one password has an invalid length.
        * Disallowed characters in password(s).
        * Identical old and new passwords.
        * Incorrect old password.
        * Invalid Bearer token format.
    * HTTP 401 Unauthorized: The provided authentication token does not map to a user.
    * HTTP 500 Internal Server Error: An error has occurred.

### POST /account/clear-tokens
Removes all active authentication tokens for a user account. This includes the token provided in this HTTP request.

* Authentication: Bearer
* Expected JSON payload: None
* Possible responses:
    * HTTP 200 OK: Success
    * HTTP 400 Bad Request: Invalid Bearer token format.
    * HTTP 401 Unauthorized: The provided authentication token does not map to a user.
    * HTTP 500 Internal Server Error: An error has occurred.

### GET /chat/rooms
Retrieve a list of rooms that the logged in user are members of.

* Authentication: Bearer
* Expected JSON payload: None
* Possible responses:
    * HTTP 200 OK:
    ```json
    {
        rooms: [<id and name>, ...]
    }
    ```
    * HTTP 400 Bad Request: Invalid Bearer token format.
    * HTTP 401 Unauthorized: The provided authentication token does not map to a logged in user.
    * HTTP 500 Internal Server Error: An error has occurred.

### POST /chat/create-room
Create a chat room with a provided name and become the first member.

* Authentication: Bearer
* Expected JSON payload:
```json
{
    "room_name": <room name>
}
```
* Possible responses:
    * HTTP 200 OK: Success
    * HTTP 400 Bad Request:
        * Invalid room name length.
        * Disallowed characters in room name.
        * Invalid token format
    * HTTP 401 Unauthorized:
        * The provided authentication token does not map to a user.
    * HTTP 500 Internal Server Error: An error has occurred.

### PUT /chat/{room_id}/change-name
Change the name of the room specified by the `room_id` parameter. Only rooms that the logged in user are a part of can be changed.

* Authentication: Bearer
* Expected JSON payload:
```json
{
    "room_name": <room name>
}
```
* Possible responses:
    * HTTP 200 OK: Success
    * HTTP 400 Bad Request:
        * Invalid room name length.
        * Disallowed characters in room name.
        * Invalid token format
    * HTTP 401 Unauthorized:
        * The provided authentication token does not map to a user.
        * The logged in user is not a member of the specified room.
    * HTTP 500 Internal Server Error: An error has occurred.

### GET /chat/{room_id}/members
List the usernames of members in the room specified by the `room_id` parameter. Only rooms that the logged in user are a part of can be requested.

* Authentication: Bearer
* Expected JSON payload: None
* Possible responses:
    * HTTP 200 OK: Success
    * HTTP 400 Bad Request: Invalid token format
    * HTTP 401 Unauthorized:
        * The provided authentication token does not map to a user.
        * The logged in user is not a member of the specified room.
    * HTTP 500 Internal Server Error: An error has occurred.

### POST /chat/{room_id}/manage-user
Add or remove a user from a chat room specified by the `room_id` parameter.

The logged in user must be a member of the room that the request specifies. I.O.W, the user is only authorised to make this request for rooms they are in.

* Authentication: Bearer
* Expected JSON payload:
```json
{
    "username": <username>,
    "action": "AddUser" | "RemoveUser" 
}
```
* Possible responses:
    * HTTP 200 OK: Success
    * HTTP 400 Bad Request:
        * Invalid token format.
        * Bad `username` provided in JSON payload.
        * Bad `action` provided in JSON payload.
    * HTTP 401 Unauthorized:
        * The provided authentication token does not map to a user.
        * The logged in user is not a member of the specified room.
    * HTTP 500 Internal Server Error: An error has occurred.

### GET /chat/{room_id}/{offset}/{limit}
Retrieve a list of messages from the chat room specified by `room_id`. The returned list of messages are a window/slice of messages in the chat room. The window/slice begins at the `offset` parameter position, and contains/has its size defined by the `limit` parameter. The most recent message in the chat room has an offset of 0.

Note: The oldest message in the specified window/slice is first in the response, with the newest/latest message being at the end of the window/slice.

* Authentication: Bearer
* Expected JSON payload: None
* Possible responses:
    * HTTP 200 OK:
    ```json
    {
        "messages": [
            {
                "id": <message id>,
                "room_id": <room id>,
                "sender_id": <sender user id>,
                "body": <message body/text>,
                "time_sent": <date & time in UTC time>
            },
            {
                ...
            },
            ...
        ]
    }
    ```
    * HTTP 400 Bad Request:
        * Invalid token format.
        * The limit parameter was 0 (describing a window/slice of size 0).
    * HTTP 401 Unauthorized:
        * The provided authentication token does not map to a user.
        * The logged in user is not a member of the specified room.
    * HTTP 500 Internal Server Error: An error has occurred.

### POST /chat
Send a message in a chat room.

* Authentication: Bearer
* Expected JSON payload:
```json
{
    "room_id": <room id>,
    "body": <body text>
}
```
* Possible responses:
    * HTTP 200 OK: Success
    * HTTP 400 Bad Request:
        * Invalid token format.
        * Extra fields were populated.
    * HTTP 401 Unauthorized:
        * The provided authentication token does not map to a user.
        * The logged in user is not a member of the specified room.
    * HTTP 500 Internal Server Error: An error has occurred.
