<script lang="ts">
    export let text: string;
</script>

<button on:click class="flex flex-col items-center bg-neutral-700 py-2 px-5 rounded hover:bg-neutral-700/60">
    <slot/>
    {text}
</button>
