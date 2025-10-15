<script lang="ts">
  export let title: string;
</script>

<div
  class="flex flex-col items-center gap-2 bg-primary-foreground p-4 rounded w-full"
>
  <span>{title}</span>
  <slot />
</div>
