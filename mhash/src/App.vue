<script setup lang="ts">
import Equip from '@/components/Equip.vue'
import { EquipSlot } from '@/data/equip.ts'
import WeaponSharpness from '@/components/WeaponSharpness.vue'
</script>

<template>
  <main>
    <WeaponSharpness id="s1" :amounts="[100, 50, 50]"></WeaponSharpness>
    <WeaponSharpness id="s2" :amounts="[120, 30, 30, 60, 50, 60]"></WeaponSharpness>

    <div class="equips">
      <!-- TODO: Weapon component -->
      <Equip eslot="Primary" :decos="[1, 1, 1]"></Equip>
      <Equip eslot="Secondary" :decos="[1, 1]"></Equip>

      <Equip :eslot="EquipSlot.HEAD" :decos="[1]"></Equip>
      <Equip :eslot="EquipSlot.CHEST" :decos="[2]"></Equip>
      <Equip :eslot="EquipSlot.ARMS" :decos="[3]"></Equip>
      <Equip :eslot="EquipSlot.WAIST" :decos="[2, 1]"></Equip>
      <Equip :eslot="EquipSlot.LEGS" :decos="[3, 2]"></Equip>
      <Equip :eslot="EquipSlot.CHARM" :decos="[3, 2, 1]"></Equip>
      TODO: Mantle 1 TODO: Mantle 2
    </div>
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

.equips {
  display: flex;
  flex-direction: column;
}

.equips > *:not(:last-child) {
  border-bottom: 1px solid var(--color-separator);
}
</style>
