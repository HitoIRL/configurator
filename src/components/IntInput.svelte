<script lang="ts">
    import { formatName } from "../helpers";
  
    export let name: string;
    export let value: number;

    function onKeyDown(event: KeyboardEvent) {
        // checking if user tries to set decimal value
        if (event.key === ".") {
            event.preventDefault();
        }
    }
</script>

<div class="flex items-center gap-3">
    <input on:keydown={onKeyDown} class="input bg-neutral-700 px-4 py-3 rounded outline-none transition-colors focus:bg-neutral-700/60" type="number" pattern=/^[0-9]+$/ id={name} {name} {value}>
    <label class="text-xl" for={name}>{formatName(name)}</label>
</div>

<style>
    .input::-webkit-outer-spin-button,
    .input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    .input[type=number] {
        -moz-appearance: textfield;
    }
</style>
