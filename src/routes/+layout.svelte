<script>
  import Header from '$lib/components/header/header.svelte';
  import NavSidebar from '$lib/components/nav-sidebar/nav-sidebar.svelte';
  import { ModeWatcher } from 'mode-watcher';
  import '../app.css';
  export let title = 'Home';
</script>

<ModeWatcher />
<div class="grid grid-cols-[auto_1fr] min-h-screen">
  <NavSidebar />
  <div class="flex flex-col flex-1">
    <Header {title} />
    <main class="flex-1">
      <slot />
    </main>
  </div>
</div>
