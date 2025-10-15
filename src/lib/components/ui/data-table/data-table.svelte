<script lang="ts">
  import * as Table from "$lib/components/ui/table";

  export let columns: { key: string; header: string; width?: string }[] = [];
  export let rows: Record<string, any>[] = [];
  export let maxHeight = "300px";
</script>

<div class="relative rounded-xl border overflow-hidden h-full">
  <div class="overflow-hidden h-full flex flex-col">
    <div class="relative flex-1 min-h-28">
      <Table.Root>
        <Table.Header class="sticky top-0 bg-background z-10">
          <Table.Row>
            {#each columns as column}
              <Table.Head class={column.width}>{column.header}</Table.Head>
            {/each}
          </Table.Row>
        </Table.Header>
      </Table.Root>
      <div
        class="overflow-auto absolute inset-0 top-[37px]"
        style="max-height: {maxHeight}"
      >
        <Table.Root>
          <Table.Body>
            {#each rows as row}
              <Table.Row>
                {#each columns as column}
                  <Table.Cell class={column.width}>{row[column.key]}</Table.Cell
                  >
                {/each}
              </Table.Row>
            {/each}
          </Table.Body>
        </Table.Root>
      </div>
    </div>
  </div>
</div>
