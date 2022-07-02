<template>
    <div class="container">
        <div class="sub-container">
            <TodoItemComponent :key="todo" v-for="todo in todos" :todo="todo" @check="changeStatus(todo)" @delete="deleteTodo(todo)"/>
        </div>
        <input type="text" id="input" v-model="inputText" />
        <button @click="createTodo()">Create</button>
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { TodoItem } from './models/TodoItem';
import TodoItemComponent from './components/TodoItem.vue';
import TodoService from "./services/TodoService";

@Options({
    components: {
        TodoItemComponent
    },
})
export default class App extends Vue {
    todos: TodoItem[] = [];
    inputText = "";

    async changeStatus(todo: TodoItem) {
        todo.status = !todo.status;
        this.sortTodosByStatus();

        await TodoService.updateTodo(todo);
    }

    async deleteTodo(todo: TodoItem) {
        this.todos.splice(this.todos.indexOf(todo), 1);
        
        await TodoService.deleteTodo(todo.id);
    }

    async createTodo() {
        if(this.inputText == "")
            return;

        await TodoService.createTodo(this.inputText);

        await this.getTodos();
        this.inputText = "";
    }

    async getTodos() {
        this.todos = await TodoService.getAllTodos();
        this.sortTodosByStatus();
    }

    sortTodosByStatus() {
        this.todos = this.todos.sort((t1, t2) => +t1.status - +t2.status)
    }

    created() {
        this.getTodos();
    }
}
</script>

<style lang="scss">
#app {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
}

.container {
    position: relative;
    background-color: rgb(32, 32, 32);
    border-radius: 15px;

    margin: 20px auto;
    padding: 20px;
    
    width: 40vw;
    height: 80vh;

    .sub-container {
        height: 95%;
    }
}

.sub-container {
    overflow-y: scroll;
}

.sub-container::-webkit-scrollbar {
    display: none;
}

#input {
    border: none;
    border-radius: 10px;
    padding: 10px;
    background-color: #555;
    color: white;
}

button {
    display: float;
    float: right;
    border: none;
    border-radius: 10px;
    padding: 10px;
    margin-left: 10px;
    background-color: #555;
    color: white;
}

@media only screen and (min-width: 1024px) {
    .container {
        width: 30vw;
    }

    #input {
        width: 75%;
    }
}

@media only screen and (max-width: 1200px) {
    .container {
        width: 40vw;
    }

    #input {
        width: 75%;
    }
}

@media only screen and (max-width: 900px) {
    .container {
        width: 65vw;
    }

    #input {
        width: 80%;
    }
}

@media only screen and (max-width: 768px) {
    .container {
        width: 80vw;
    }
}

@media only screen and (max-width: 560px) {
    .container {
        width: 80vw;
    }

    #input {
        width: 70%;
    }
}
</style>
