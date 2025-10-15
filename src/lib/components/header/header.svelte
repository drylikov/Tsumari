<script>
  import LightSwitch from '../light-switch/light-switch.svelte';

  export let title = 'Home';
</script>

<header class="bg-background p-4 flex justify-between items-center border-b">
  <h1 class="text-lg font-semibold">{title}</h1>
  <div class="w-full flex justify-end">
    <LightSwitch />
  </div>
</header>
