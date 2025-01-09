# API endpoints

## Available endpoints
* [`GET  /health`](#get-health)
* [`POST /register`](#post-register)
* [`POST /login`](#post-login)
* [`POST /change-password`](#post-change-password)
* [`POST /clear-tokens`](#post-clear-tokens)

### GET /health

Poll the server to see if it is running.

* Authentication: None
* Expected JSON payload: None
* Possible responses:
    * HTTP 200 OK: The server is running

### POST /register
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

### POST /login
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

### POST /change-password
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

### POST /clear-tokens
Removes all active authentication tokens for a user account. This includes the token provided in this HTTP request.

* Authentication: Bearer
* Expected JSON payload: None
* Possible responses:
    * HTTP 200 OK: Success
    * HTTP 400 Bad Request: Invalid Bearer token format.
    * HTTP 401 Unauthorized: The provided authentication token does not map to a user.
    * HTTP 500 Internal Server Error: An error has occurred.