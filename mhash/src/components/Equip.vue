<script setup lang="ts">
const DECO_COUNT = 3
const props = defineProps(['eslot', 'decos'])
const decosDisabled = DECO_COUNT - props.decos.length
</script>

<template>
  <div class="equip">
    <div class="trigger">
      <div class="icon"></div>
      <div class="text">
        <div class="slot">{{ eslot }}</div>
        <div class="name">NAME HERE</div>
      </div>
    </div>
    <div class="decos">
      <div v-for="x in decos" class="deco">
        <div class="icon"></div>
        <div class="name">{{ x }}</div>
      </div>
      <div v-for="x in decosDisabled" class="deco disabled">
        <div class="icon">&horbar;</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.equip {
  display: flex;
  align-content: center;
  align-items: center;
  gap: 0.5em;
  padding: 0.5em;
}

.trigger {
  flex: 1 1 auto;
  display: flex;
  align-content: center;
  align-items: center;
  gap: 0.5em;
  padding: 0.5em;
  cursor: pointer;
}

.trigger:hover {
  background: rgba(81, 227, 57, 0.26);
  box-shadow: 0 0 8px 6px black inset;
}

.equip > .trigger > .icon {
  flex: 0 0 auto;
  width: 2.4em;
  height: 2.4em;

  background-color: var(--color-background-alt);
  border: 1px solid var(--color-border);
  border-radius: 6px;

  background-image: url('src/assets/sword-thin-svgrepo-com.svg');
  background-size: 100%;
  background-position: center center;
  background-repeat: no-repeat;
}

.trigger:hover > .icon {
  border-color: var(--color-border-hover);
  box-shadow: 0 0 3px 1px var(--color-border-hover);
}

.text {
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
}

.text > .slot {
  color: var(--color-subtext);
  font-size: 0.8em;
  line-height: 1em;
}

.decos {
  flex: 1 1 30%;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.decos > .deco {
  cursor: pointer;
  display: flex;
  align-content: center;
  align-items: center;
  gap: 4px;

  background: var(--color-background-alt);
  border: 1px solid var(--color-border);
  border-radius: 4px;
  padding: 2px;
}

.decos > .deco > .icon {
  height: 1.4em;
  width: 1.4em;
  background-image: url('src/assets/gem-svgrepo-com.svg');
  background-size: 100%;
  background-position: center center;
  background-repeat: no-repeat;
}

.decos > .deco > .name {
  line-height: 1.2em;
}

.decos > .deco:hover {
  border-color: var(--color-border-hover);
  box-shadow: 0 0 1px 2px var(--color-border-hover);
}

.decos > .deco.disabled {
  cursor: default;
  border-color: transparent;
}

.decos > .deco.disabled:hover {
  box-shadow: none;
}

.decos > .deco.disabled > .icon {
  background: none;
  text-align: center;
}
</style>
