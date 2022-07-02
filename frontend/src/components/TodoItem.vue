<template>
    <div class="todo-item">
        <!-- <input type="checkbox" :checked="todo.status" @click="this.$emit('click')"/> -->
        <CheckboxComponent :checked="todo.status" @click="this.$emit('check')"/> 
        <div class="todo-text">{{ todo.title }}</div>
        <button class="delete-todo" @click="this.$emit('delete')">X</button>
    </div>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { TodoItem } from "../models/TodoItem";
import CheckboxComponent from './Checkbox.vue';

@Options({
    props: { todo: Object },
    emits: [
        "check",
        "delete"
    ],
    name: "TodoItem",
    components: {
        CheckboxComponent
    }
})

export default class TodoItemComponent extends Vue {
    todo!: TodoItem
}
</script>

<style lang="scss" scoped>
.todo-item {
    display: flex;
    color: white;

    &:hover > .delete-todo {
        display: inline;
    }
}

.todo-text {
    margin-left: 15px;
}

.delete-todo {
    padding: 5px;
    width: 30px;
    height: 30px;
    color: red;
    display: none;

    &:hover {
        background-color: #333;
    }
}

div {
    line-height: 25px;
    margin-bottom: 5px;
}
</style>