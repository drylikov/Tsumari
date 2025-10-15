<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import DataTable from "$lib/components/ui/data-table/data-table.svelte";

  const rows = [
    { host: "localhost", port: "22" },
    { host: "localhost", port: "22" },
    { host: "localhost", port: "22" },
    { host: "localhost", port: "22" },
    { host: "localhost", port: "22" },
    { host: "localhost", port: "22" },
    { host: "localhost", port: "22" },
  ];
</script>

<main class="flex flex-col h-full gap-4 p-4">
  <Card.Root class="flex flex-col flex-1">
    <Card.Header>
      <Card.Title>Connections</Card.Title>
      <Card.Description>Add, edit or remove connections</Card.Description>
    </Card.Header>
    <Card.Content class="flex-1">
      <DataTable
        columns={[
          { key: "host", header: "Host", width: "w-[200px]" },
          { key: "port", header: "Port", width: "w-[100px]" },
        ]}
        {rows}
      />
    </Card.Content>
  </Card.Root>
</main>
