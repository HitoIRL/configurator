<script lang="ts">
  import { formatName } from "../helpers";

    export let name: string;
    export let value: string;
</script>

<div class="flex items-center gap-3">
    <input class="bg-neutral-700 px-4 py-3 rounded outline-none transition-colors w-96 focus:bg-neutral-700/60" type="text" id={name} {name} {value}>
    <label class="text-xl" for={name}>{formatName(name)}</label>
</div>