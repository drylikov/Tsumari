<script lang="ts">
  import { Progress as ProgressPrimitive } from 'bits-ui';
  import { cn } from '$lib/utils.js';

  type $$Props = ProgressPrimitive.Props;

  let className: $$Props['class'] = undefined;
  export let max: $$Props['max'] = 100;
  export let value: $$Props['value'] = undefined;
  export let size: number = 160; // Size of the circle in pixels
  export let strokeWidth: number = 8; // Width of the progress bar
  export { className as class };

  $: radius = (size - strokeWidth) / 2;
  $: circumference = 2 * Math.PI * radius;
  $: offset = circumference - ((value ?? 0) / (max ?? 1)) * circumference;
</script>

<ProgressPrimitive.Root class={cn('relative', className)} {...$$restProps}>
  <div class="relative" style={`width: ${size}px; height: ${size}px`}>
    <!-- Background circle -->
    <svg class="rotate-[-90deg]" width={size} height={size}>
      <circle
        class="text-primary/20"
        stroke="currentColor"
        fill="none"
        stroke-width={strokeWidth}
        r={radius}
        cx={size / 2}
        cy={size / 2}
      />
      <!-- Progress circle -->
      <circle
        class="text-primary transition-all duration-300 ease-in-out"
        stroke="currentColor"
        fill="none"
        stroke-width={strokeWidth}
        stroke-dasharray={circumference}
        stroke-dashoffset={offset}
        stroke-linecap="round"
        r={radius}
        cx={size / 2}
        cy={size / 2}
      />
    </svg>

    <!-- Optional: Add content in the center -->
    <div class="absolute inset-0 flex items-center justify-center">
      <slot>
        <span class="text-sm font-medium">
          {(((value ?? 0) / (max ?? 1)) * 100).toFixed(1)}%
        </span>
      </slot>
    </div>
  </div>
</ProgressPrimitive.Root>
