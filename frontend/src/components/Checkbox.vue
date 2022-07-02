<template>
    <span class="outer-checkbox" @click="this.$emit('click')">
        <span v-if="checked" class="inner-checkbox"></span>
    </span>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';

@Options({
    props: {
        checked: Boolean
    },
    emits: ["click"]
})

export default class CheckboxComponent extends Vue {
    checked!: boolean;
}
</script>

<style lang="scss" scoped>
    $size: 18px;

    .outer-checkbox {
        display: inline-block;
        width: $size;
        height: $size;
        border: 2px solid white;
        border-radius: 7px;
        position: relative;

        padding: 2px;
        line-height: 40px;

        .inner-checkbox {
            position: absolute;
            left: 4px;
            top: 3.5px;
            display: block;
            width: 14px;
            height: 14px;
            background: rgb(230, 230, 230);
            border-radius: 5px;
        }
    }
</style>