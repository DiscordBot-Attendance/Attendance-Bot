pub const HELP_MESSAGES: &str = "
**Attendance Bot Commands**  

Hi! I'm the Attendance Bot. Here are the commands you can use:

🛠 **Admin Commands**  
🔹 `!AB register {password}`  → Register as Admin team  

👥 **Team Management**  
🔹 `!AB create_team {team_name}`  → Create a new team
🔹 `!AB show_team`  → Show existing teams  

👤 **Member Management**  
🔹 `!AB add_member {team_name} @member_name`  → Add a member to a team  
🔹 `!AB show_member {team_name}`  → Show members of a team  
🔹 `!AB show_member_attendance {team_name}`  → Show attendance for a team's members  

⏳ **Attendance Tracking**  
🔹 `!AB check_in {team_name} {status}`  → Start session (Check-in)  
🔹 `!AB check_out {team_name}`  → End session (Check-out)  

⚠️ **Note:** Ensure you have the correct permissions and passwords for Admin-related commands.
";
