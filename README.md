# YewChat 💬

# Experiment 3.1: Original code

## How to run:

First, start the WebSocket server (SimpleWebsocketServer) in one terminal:
```bash
npm start
```

Then, start the YewChat frontend in another terminal:
```bash
npm start
```

Open http://localhost:8000 in the browser, enter a username, and start chatting.

---

## Screenshot - Server Terminal:

![Experiment 3.1 - Server Terminal](assets/images/3.1:%20Original%20code%20[server%20terminal].png)

## Screenshot - Client Terminal:

![Experiment 3.1 - Client Terminal](assets/images/3.1:%20Original%20code%20[client%20terminal].png)

## Screenshot - Browser:

![Experiment 3.1 - Browser](assets/images/3.1:%20Original%20code%20[browser].png)

---

## What happens:

The SimpleWebsocketServer listens on port 8080 and logs ws connected for every client that joins. The YewChat frontend is a Rust/Yew application compiled to WebAssembly and served via webpack on port 8000. When two browser tabs are opened with different usernames (aldo and aldo-clone), both users appear in the Users panel. Messages typed in one tab are broadcast to all connected clients in real time through the WebSocket connection, demonstrating full duplex asynchronous communication between the browser and the server.
