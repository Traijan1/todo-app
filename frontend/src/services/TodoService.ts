import { TodoItem } from "@/models/TodoItem";

class TodoService {
    private baseUrl = "http://192.168.178.98:8000/api";
    private defaultHeader = {
        "Content-Type": "application/json"
    }

    async deleteTodo(id: number) {
        await fetch(`${this.baseUrl}/todo?id=${id}`, {
            method: "DELETE",
            mode: "cors",
            headers: this.defaultHeader
        })
    }

    async createTodo(title: string) {
        const newTodo = {
            title: title,
            status: 0 // <-- False for the backend
        };

        await fetch(`${this.baseUrl}/todo`, {
            method: "POST",
            mode: "no-cors",
            headers: this.defaultHeader,
            body: JSON.stringify(newTodo)
        })
    }

    async getAllTodos(): Promise<TodoItem[]> {
        const response = await fetch(`${this.baseUrl}/todos`);
        return await response.json() as TodoItem[];
    }

    async updateTodo(todo: TodoItem): Promise<void> {
        await fetch(`${this.baseUrl}/changeTodo`, {
            method: "POST",
            mode: "no-cors",
            headers: this.defaultHeader,      // |--- Backend has to use numbers for boolean representation
            body: JSON.stringify(todo).replace("true", "1").replace("false", "0")
        });
    }
}

export default new TodoService();