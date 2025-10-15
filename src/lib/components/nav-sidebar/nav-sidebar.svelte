<script lang="ts">
  import { Cable, SquareActivity } from "lucide-svelte";
  import { Button } from "../ui/button";
  import * as Tooltip from "../ui/tooltip";
  import { page } from "$app/stores";

  let path = $state($page.url.pathname);

  $effect(() => {
    path = $page.url.pathname;
  });
</script>

<aside
  class="bg-background w-16 flex flex-col items-center space-y-2 py-4 border-r"
>
  <Tooltip.Root>
    <Tooltip.Trigger class="w-10 h-10">
      <Button
        class="w-10 h-10"
        href="/"
        variant={path === "/" ? "default" : "ghost"}
        size="icon"><Cable /></Button
      >
    </Tooltip.Trigger>
    <Tooltip.Content>
      <p>Connections</p>
    </Tooltip.Content>
  </Tooltip.Root>

  <Tooltip.Root>
    <Tooltip.Trigger class="w-10 h-10">
      <Button
        class="w-10 h-10"
        href="/monitor"
        variant={path === "/monitor" ? "default" : "ghost"}
        size="icon"><SquareActivity /></Button
      >
    </Tooltip.Trigger>
    <Tooltip.Content>
      <p>Monitor</p>
    </Tooltip.Content>
  </Tooltip.Root>
</aside>
