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

---

# Experiment 3.2: Be Creative!

In this experiment, I transformed the YewChat client into a sophisticated, minimal, and modern communication tool.

### New Features:

- Smart Translate

Added a built-in translation toggle for every message. With one click, messages are processed through a mock translation engine (currently demonstrating with reversed text) to show how a global chat would look.

- Elegant UI Redesign

Replaced loud gradients with a professional Slate and Indigo theme. This provides a cleaner look that emphasizes content over distracting colors.

- Minimalist Login

A clean, high-contrast white card design with a dedicated brand icon and improved typography. This makes the first interaction with the app feel more premium.

- Modern Chat Bubbles

Own messages now appear in dark bubbles aligned to the right, while received messages use crisp white bubbles on the left. This features soft shadows and refined spacing for better legibility.

- Glassmorphism Header

Added a YewChat brand title with a blue online indicator dot and a semi-transparent, blurred header for a premium feel.

- Enhanced Sidebar

Renamed sidebar to ONLINE USERS with grayscale avatars that come to life on hover and green status indicators to show active presence.

### UX Improvements:

- Interactive Translation

Hover over any message to reveal the Translate action. This keeps the interface clean while keeping powerful features accessible.

- Improved Responsiveness

Better spacing and padding for a comfortable reading experience. The layout adapts better to different message lengths.

- Keyboard Support

Full Enter key support for seamless message sending. This allows for a much faster conversational flow.

- Persistent History

Message history is now preserved across login and logout sessions by utilizing a global context. This solves the problem of losing data when switching users.

- Echo Fix

Eliminated duplicate self-messages for a cleaner experience. Users no longer see their own broadcasted messages returned to them in the chat log.

- Sign Out button

Added a SIGN OUT button at the bottom left of the sidebar so users can leave the session cleanly. This replaces the need to manually navigate away from the chat page.

---

## Screenshot - Server Terminal:

![Experiment 3.2 - Server Terminal](assets/images/3.2:%20Be%20Creative!%20[server%20terminal].png)

## Screenshot - Client Terminal:

![Experiment 3.2 - Client Terminal](assets/images/3.2:%20Be%20Creative!%20[client%20terminal].png)

## Screenshot - Browser:

![Experiment 3.2 - Browser](assets/images/3.2:%20Be%20Creative!%20[browser].png)

---

## Explanation:

The creative modifications focused on improving the user experience without breaking the underlying WebSocket functionality. The dark message bubbles for own messages follow a modern chat convention that makes conversations much easier to follow. The Session active as label in the header solves a common confusion in multi-tab usage where users forget which username they logged in with. The SIGN OUT button was added because the original code had no way to return to the login page without manually navigating, which felt incomplete. All changes were made in the Yew component files using Tailwind CSS utility classes, keeping the Rust and WebAssembly architecture intact.
