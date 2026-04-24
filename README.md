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

The SimpleWebsocketServer listens on port 8080 and logs ws connected for every client that joins. The YewChat frontend is a Rust and Yew application compiled to WebAssembly and served via webpack on port 8000. When two browser tabs are opened with different usernames (aldo and aldo-clone), both users appear in the Users panel. Messages typed in one tab are broadcast to all connected clients in real time through the WebSocket connection, demonstrating full duplex asynchronous communication between the browser and the server.

---

# Experiment 3.2: Be Creative!

## What was changed:

Several UI and UX improvements were made to the YewChat webclient to make it more polished and modern:

- Redesigned header

Added a YewChat brand title with a blue online indicator dot and a Session active as label so users always know which account they are logged in as.

- Renamed sidebar label

Changed Users to ONLINE USERS with a green dot indicator next to each username to show active presence.

- Distinct message bubbles

Own messages now appear in dark bubbles aligned to the right, while received messages use light bubbles on the left, making it visually clear which messages belong to the current user.

- Improved input placeholder

Changed to Compose a message for a more conversational feel.

- Sign Out button

Added a SIGN OUT button at the bottom left of the sidebar so users can leave the session cleanly.

---

## Screenshot - Server Terminal:

![Experiment 3.2 - Server Terminal](assets/images/3.2:%20Be%20Creative!%20[server%20terminal].png)

## Screenshot - Client Terminal:

![Experiment 3.2 - Client Terminal](assets/images/3.2:%20Be%20Creative!%20[client%20terminal].png)

## Screenshot - Browser:

![Experiment 3.2 - Browser](assets/images/3.2:%20Be%20Creative!%20[browser].png)

---

## Explanation:

The creative modifications focused on improving the user experience without breaking the underlying WebSocket functionality. The dark message bubbles for own messages follow a modern chat convention (similar to WhatsApp and iMessage) that makes conversations much easier to follow. The Session active as label in the header solves a common confusion in multi tab usage where users forget which username they logged in with. The SIGN OUT button was added because the original code had no way to return to the login page without manually navigating, which felt incomplete. All changes were made in the Yew component files using Tailwind CSS utility classes, keeping the Rust and WebAssembly architecture intact.

---

# Bonus: Technical Refinements and Bug Fixes

In addition to the UI changes, I implemented several advanced features to improve the stability and logic of the application.

## How it was done:

- Smart Translate

I added a built-in translation toggle for every message. When triggered, the message is processed through a mock translation engine to demonstrate how a globalized chat interface would function.

- Persistent History

I refactored the application state to move the message history into a global context. This ensures that chat logs are preserved even when a user signs out and signs back in.

- Echo Fix

I updated the client logic to prevent the browser from displaying a duplicate copy of messages sent by the user. The client now intelligently handles its own broadcasted messages.

- Ghost Connection Eviction

I implemented a session management system on the server that automatically identifies and closes old connections when a nickname is reclaimed. This prevents the double message bug that occurs when users log back in without a clean disconnect.

---

## Screenshot - Rust WebSocket Server:

![Bonus - Rust Server](assets/images/Bonus:%20Rust%20Websocket%20server%20for%20YewChat!%20server.png)

## Screenshot - YewChat Browser:

![Bonus - Browser](assets/images/Bonus:%20Rust%20Websocket%20server%20for%20YewChat!%20browser.png)

---

## Explanation:

The screenshots demonstrate the successful integration of the Rust asynchronous server with the YewChat frontend. The server terminal shows active logs for user registration and message broadcasting while handling the JSON protocol. The browser screenshot displays the modern chat interface correctly rendering messages received from the Rust backend. This setup confirms that the ghost eviction and persistent state logic are functioning as intended within the Rust environment.
