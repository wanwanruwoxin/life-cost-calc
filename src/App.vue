<script setup lang="ts">

const text = ref("");
</script>

<template>
  <main class="container">
    <QLayout view="hHh Lpr lFf">
      <QHeader>
        <p>Life Cost Calculator</p>
      </QHeader>
      <QPageContainer>
        <QPage flex="~ content-center justify-center">
          <div w-60 flex="~ col" border="2px solid red" p-2>
            <QInput v-model="text" label="说明" />
            <QInput v-model="text" label="金额" />
            <QInput v-model="text" label="频率" />

            <QBtn color="primary" label="Primary" />
          </div>
        </QPage>
      </QPageContainer>
    </QLayout>
  </main>
</template>

<style scoped></style>
