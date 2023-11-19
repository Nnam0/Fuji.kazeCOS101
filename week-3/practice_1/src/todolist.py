import json

def display_todos():
    with open('todos.json', 'r') as f:
        todos = json.load(f)
        for todo in todos:
            print(f"{todo['id']}) {todo['task']}")

def add_todo(task):
    with open('todos.json', 'r') as f:
        todos = json.load(f)

    id = todos[-1]['id'] + 1 if todos else 1
    new_todo = {'id': id, 'task': task}
    todos.append(new_todo)

    with open('todos.json', 'w') as f:
        json.dump(todos, f, indent=2)

def delete_todo(id):
    with open('todos.json', 'r') as f:
        todos = json.load(f)

    for i, todo in enumerate(todos):
        if todo['id'] == id:
            del todos[i]
            break

    with open('todos.json', 'w') as f:
        json.dump(todos, f, indent=2)

while True:
    print("\nTo Do List")
    print("-------------")
    print("1) View tasks")
    print("2) Add task")
    print("3) Delete task")
    print("4) Quit")
    choice = input("Enter your choice: ")

    if choice == '1':
        display_todos()
    elif choice == '2':
        task = input("Enter task: ")
        add_todo(task)
        print(f"Added task: {task}")
    elif choice == '3':
        id = int(input("Enter task id to delete: "))
        delete_todo(id)
        print(f"Deleted task with id: {id}")
    elif choice == '4':
        break
    else:
        print("Invalid choice. Please try again.")