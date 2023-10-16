<script lang="ts">
  import { formatName } from "../helpers";

    export let name: string;
    export let value: boolean;
</script>

<label class="relative flex items-center cursor-pointer pl-10 select-none text-xl">
    <input class="checkbox hidden" type="checkbox" id={name} {name} checked={value}>
    <span class="absolute top-0 left-0 w-7 h-7 bg-neutral-700 border-4 border-neutral-700 rounded transition-colors"></span>
    {formatName(name)}
</label>

<style lang="postcss">
    .checkbox:checked ~ span {
        background-color: theme(colors.violet.700);
    }
</style>
