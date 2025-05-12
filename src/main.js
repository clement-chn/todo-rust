const { invoke } = window.__TAURI__.core;

let todoInputEl;
let priorityInputEl;
let todoEl;
let todoListEl;

window.addEventListener("DOMContentLoaded", () => {
  todoInputEl = document.querySelector("#task-input");
  priorityInputEl = document.querySelector("#task-priority");
  todoEl = document.querySelector("#new-task");
  todoListEl = document.querySelector("#todo-list");

  loadTodos();

  document.querySelector("#add-task-form").addEventListener("submit", (e) => {
    e.preventDefault();

    const todoName = todoInputEl.value;
    const priority = priorityInputEl.value;

    invoke("save_name_to_json", { todoName, priority })
      .then(() => {
        console.log("Tâche ajoutée !");
        loadTodos();
      })
      .catch((err) => {
        console.error("Erreur lors de l'ajout de la tâche :", err);
      });
  });
});

async function loadTodos() {
  try {
    const todos = await invoke("get_todos");
    const todoArray = JSON.parse(todos);

    todoListEl.innerHTML = "";

    todoArray.forEach((todo, index) => {
      const li = document.createElement("li");
      li.textContent = `${todo.todo_name} ${todo.priority}`;

      const deleteButton = document.createElement("button");
      deleteButton.textContent = "Supprimer";
      deleteButton.id = `delete-${index}`;
      deleteButton.addEventListener("click", () => {
        deleteTodo(index);
      });

      li.appendChild(deleteButton);

      todoListEl.appendChild(li);
    });
  } catch (err) {
    console.error("Erreur lors du chargement des tâches :", err);
  }
}

async function deleteTodo(index) {
  try {
    await invoke("delete_todo", { index });
    console.log(`Tâche ${index} supprimée !`);
    loadTodos();
  } catch (err) {
    console.error("Erreur lors de la suppression de la tâche :", err);
  }
}
