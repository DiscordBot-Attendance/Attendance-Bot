# Attendance Bot - Discord & API (Beta)

## 📌 Overview

Attendance Bot is a Discord bot designed for managing team attendance, tracking check-ins and check-outs, and handling team memberships. This bot allows admins to create teams, add members, and monitor attendance through simple Discord commands and a RESTful API.

## ⚡ Features

- Admin registration & authentication
- Team creation and member management
- Attendance check-in & check-out
- View team and member attendance records
- REST API for external integrations (Beta)

## 🛠 Setup & Installation

### Prerequisites

Before running the bot and API, ensure you have the following:

- Rust installed (latest stable version)
- PostgreSQL installed and running
- Discord bot token (from the Discord Developer Portal)

### Installation Steps

1. Clone the repository:
   ```sh
   git clone https://github.com/your-repo/attendance-bot.git
   cd attendance-bot
   ```
2. Set up environment variables in a `.env` file:
   ```env
   DISCORD_TOKEN=your_bot_token
   DATABASE_URL=postgres://user:password@localhost/attendance_db
   JWT_SECRET=RANDOM_SENTENCE
   ```
3. Run database migrations (using Diesel ORM):
   ```sh
   diesel migration run
   ```
4. Build and run the bot and API:
   ```sh
   cargo run
   ```

## 🖥 Bot Usage

### 📌 **Bot Commands**

#### 🛠 **Admin Commands**

- `!AB register {password}` → Register as Admin team

#### 👥 **Team Management**

- `!AB create_team {team_name}` → Create a new team
- `!AB show_team` → Show existing teams

#### 👤 **Member Management**

- `!AB add_member {team_name} @member_name` → Add a member to a team
- `!AB show_members {team_name}` → Show members of a team
- `!AB show_members_attendance {team_name}` → Show attendance for a team's members

#### ⏳ **Attendance Tracking**

- `!AB check_in {team_name} {status}` → Start session (Check-in)
- `!AB check_out {team_name}` → End session (Check-out)

⚠️ **Note:** Ensure you have the correct permissions and passwords for Admin-related commands.

---

## 🌍 API Usage (Beta)

### **Base URL:**

```
Soon
```

### **Authentication**

#### **Login (POST /login)**

Authenticate and receive a token.

##### **Request:**

```json
{
  "username": "admin_discord_id",
  "password": "your_password"
}
```

##### **Response:**

```json
{
  "token": "random_token"
}
```

### **Team Management**

#### **Show Team List (GET /teams/{admin_discord_id})**

Retrieve all teams created by an admin.

##### **Response:**

```json
[
  {
    "name": "fufufafa",
    "created_at": "2025-02-21 02:36:29"
  }
]
```

### **Member Management**

#### **Show Member List (GET /members/{team_name})**

Retrieve all members in a team.

##### **Response:**

```json
[
  {
    "username": "your_name",
    "join_date": "2025-02-21"
  }
]
```

### **Attendance Tracking**

#### **Show Attendance List (GET /attendance/{team_name})**

Retrieve attendance records of members in a team.

##### **Response:**

```json
[
  {
    "username": "your_name",
    "check_in_time": "2025-02-21 02:40:46",
    "check_out_time": "2025-02-21 02:40:56",
    "status": "Present"
  }
]
```

---

## 📜 License

This project is open-source under the MIT License.

## 🚀 Future Enhancements

- Webhooks for real-time attendance updates
- Terminal-based bot management tool
- API authentication improvements

## 💡 Contributing

Contributions are welcome! Feel free to submit issues and pull requests.
