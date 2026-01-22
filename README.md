# Task Manager 📋

A simple and easy-to-use **To Do List** application that helps you organize your tasks and stay productive. This application stores all your tasks safely in a database and lets you manage them quickly.

---

## What Can You Do?

This Task Manager allows you to:

✅ **Add New Tasks** - Create tasks with a title, description, due date, and priority level  
✅ **View All Tasks** - See a list of all your tasks at a glance  
✅ **Update Tasks** - Change the status of your tasks (e.g., mark as complete)  
✅ **Delete Tasks** - Remove tasks you no longer need  

---

## Getting Started

### Before You Start

Make sure you have the following installed on your computer:

- **Rust Programming Language** (Download from [https://www.rust-lang.org/](https://www.rust-lang.org/))
- **SQLite** (usually included with most systems, or available from [https://www.sqlite.org/](https://www.sqlite.org/))

### Installation Steps

1. **Download the Project**
   - Navigate to where you downloaded this project on your computer
   ```
   cd task_manager
   ```

2. **Set Up the Database**
   - Run the database setup script to prepare everything:
   ```
   sqlite3 database < script/db_script.sql
   ```

3. **Run the Application**
   - Start the Task Manager:
   ```
   cargo run
   ```

That's it! The application will start and show you a menu.

---

## How to Use

When you start the application, you'll see a menu like this:

```
================================
     To Do List
================================
0. Exit
1. Add Task
2. View Tasks
3. Update Task (status only)
4. Delete Task
Enter your choice:
>
```

### Using Each Option

**Option 1: Add a New Task**
- Press `1` and hit Enter
- Enter your task title (e.g., "Buy groceries")
- Add a description (optional - what details do you want to remember?)
- Set a priority (1 = low, 5 = high - how important is this task?)
- Set a due date (optional - when do you need to finish it?)
- Your task is saved automatically!

**Option 2: View All Tasks**
- Press `2` and hit Enter
- See all your tasks displayed with their details
- Shows: title, description, status, priority, and due date

**Option 3: Update a Task**
- Press `3` and hit Enter
- Enter the task ID you want to update
- Change the status (e.g., from "pending" to "completed")
- Your changes are saved automatically!

**Option 4: Delete a Task**
- Press `4` and hit Enter
- Enter the task ID of the task you want to delete
- The task is removed permanently

**Option 0: Exit**
- Press `0` and hit Enter
- The application closes safely

---

## Understanding Task Information

When you create or view a task, here's what each field means:

| Field | What it is | Example |
|-------|-----------|---------|
| **ID** | A unique number for your task | 1, 2, 3 |
| **Title** | The name of your task | "Finish project" |
| **Description** | Extra details about the task | "Complete the design phase" |
| **Status** | Current state of the task | "pending", "completed" |
| **Priority** | How urgent it is (1-5) | 5 = very important, 1 = not urgent |
| **Due Date** | When the task needs to be done | "2026-01-25" |
| **Created At** | When you added the task | Automatically recorded |
| **Updated At** | Last time you changed the task | Automatically recorded |

---

## Common Questions

**Q: Where are my tasks saved?**  
A: All your tasks are saved in a local database file on your computer. They're stored safely even after you close the application.

**Q: Can I use this on my phone?**  
A: This is a command-line application for computers. You need to run it on Windows, Mac, or Linux.

**Q: Can I share my tasks with others?**  
A: Currently, this is a personal task manager. Sharing features could be added in the future!

**Q: What happens if I delete a task by mistake?**  
A: Unfortunately, deleted tasks cannot be recovered. Be careful when using the delete option.

**Q: Can I set reminders for tasks?**  
A: This version doesn't have reminders, but you can add due dates to keep track of deadlines.

---

## Troubleshooting

**Problem: The application won't start**
- Make sure Rust is installed correctly
- Try running `cargo clean` and then `cargo run` again

**Problem: Database setup fails**
- Make sure SQLite is installed on your system
- Check that the `script/db_script.sql` file exists

**Problem: I'm getting an error when adding a task**
- Make sure the database is set up (see Installation Steps)
- Check that you're entering data in the correct format

---

## Project Structure

Here's what each file does:

- **main.rs** - The heart of the application; shows the menu and runs the program
- **task.rs** - Contains all task-related functions (add, view, update, delete)
- **storage.rs** - Manages communication with the database
- **utils.rs** - Helper functions for displaying menus and messages
- **database** - Folder where your tasks are stored
- **db_script.sql** - Script that sets up the database structure

---

## Tips for Best Use

1. **Be Clear with Task Titles** - Use short, descriptive titles so you know what each task is about
2. **Set Priorities** - Mark important tasks with higher priority numbers so you focus on what matters
3. **Update Status Regularly** - Keep your task statuses updated so you know what's done and what's not
4. **Use Due Dates** - Set deadlines to stay on track and manage your time better
5. **Review Regularly** - Check your task list regularly to stay organized

---

## Need More Help?

If you encounter issues or have questions:
- Check the Troubleshooting section above
- Make sure all prerequisites are installed
- Review the How to Use section for step-by-step instructions

---

## Version

Current Version: **0.1.0**

---

**Happy Task Managing! 🚀**
